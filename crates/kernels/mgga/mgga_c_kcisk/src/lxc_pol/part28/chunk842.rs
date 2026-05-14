//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 842/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk842<F: Float>(t4816: F, t5320: F, t1929: F, t5060: F, t5277: F, t718: F, t11236: F, t740: F, t11225: F, t732: F, t640: F, t719: F, t1849: F, t1860: F, t1922: F, t979: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11717 = t4816 * t5320;
    let t11730 = t1929 * t5060;
    let t11731 = t11730 * sigma2;
    let t11763 = t5277 * t718;
    let t11769 = t11236 * t740;
    let t11774 = t732 * t11225;
    let t11807 = 1.0 / t719 / t640;
    let t11821 = t1860 * t1849;
    let t11853 = t979 * t1922;
    (t11717, t11730, t11731, t11763, t11769, t11774, t11807, t11821, t11853)
}
