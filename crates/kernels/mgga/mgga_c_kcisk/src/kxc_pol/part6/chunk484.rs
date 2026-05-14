//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 484/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk484<F: Float>(t1849: F, t702: F, t5060: F, t732: F, t1934: F, t718: F, t41: F, t642: F, t5061: F, t740: F, t745: F, t1872: F, t641: F, t79: F, t719: F, t4808: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5259 = t702 * t1849;
    let t5283 = t732 * t5060;
    let t5284 = t5283 * sigma2;
    let t5289 = t1934 * t718;
    let t5290 = t41 * t642;
    let t5315 = t5061 * t740;
    let t5320 = t740 * t745;
    let t5321 = t1872 * t5320;
    let t5322 = t79 * t641;
    let t5330 = 1.0 / t719;
    let t5344 = 0.38691203703703703703e-3 * t4808;
    (t5259, t5283, t5284, t5289, t5290, t5315, t5320, t5321, t5322, t5330, t5344)
}
