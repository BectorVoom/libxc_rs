//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 650/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk650<F: Float>(t225: F, t7910: F, t1892: F, t1955: F, t1903: F, t2022: F, t7296: F, t1882: F, t543: F, t7301: F, t545: F, t2028: F, t1904: F, t2027: F, t2030: F, t213: F, t561: F, t7245: F, t7248: F, t7279: F, t7288: F, t7291: F, t7295: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7911 = t7910 * t225;
    let t7917 = t1955 * t1892;
    let t7920 = t2022 * t1903;
    let t7921 = t7296 * t7920;
    let t7925 = t2022 * t1882 * t543;
    let t7926 = t7301 * t7925;
    let t7929 = t545 * t7910;
    let t7930 = t2028 * t7929;
    let t7933 = -t7245 + t7248 + 0.65854491829355115987e0 * t213 * t7911 * t561 - 0.65854491829355115987e0 * t7279 * t1904 + t7288 - t7291 - 0.4336814094102599731e0 * t7917 * t2030 + 0.8673628188205199462e0 * t7295 * t7921 + 0.4336814094102599731e0 * t7295 * t7926 - 0.4336814094102599731e0 * t2027 * t7930;
    (t7911, t7917, t7920, t7921, t7925, t7926, t7929, t7930, t7933)
}
