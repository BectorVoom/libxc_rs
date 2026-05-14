//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 999/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk999<F: Float>(t17705: F, t655: F, t2455: F, t4983: F, t16013: F, t7234: F, t10831: F, t9: F, t662: F, t15999: F, t5005: F, t963: F, t16009: F, t5002: F, t7219: F, t17356: F, t17360: F, t17362: F, t4989: F, t5013: F, t5034: F, t664: F, t7208: F, t7270: F, sigma2: F) -> (F, F, F) {
    let t17706 = t17705 * sigma2;
    let t17707 = t17706 * t655;
    let t17710 = t4983 * t2455;
    let t17713 = t7234 * t16013;
    let t17716 = t9 * t10831;
    let t17717 = t17716 * t662;
    let t17718 = t17717 * t15999;
    let t17721 = t963 * t5005;
    let t17722 = t17721 * t662;
    let t17723 = t17722 * t16009;
    let t17726 = t7219 * t5002;
    let t17730 = -t17356 + t17360 - t17362 - 0.10794473229706390328e0 * t4989 * t7270 + 0.5397236614853195164e-1 * t17707 * t664 - 0.14392630972941853771e0 * t17710 * t664 + 0.23987718288236422951e-1 * t5013 * t17713 + 0.55971342672551653552e-1 * t5013 * t17718 - 0.95950873152945691803e-1 * t5013 * t17723 + 0.95950873152945691806e-1 * t17726 + 0.10794473229706390328e0 * t7208 * t5034;
    (t17717, t17722, t17730)
}
