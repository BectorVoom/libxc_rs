//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1045/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1045(t352: f64, t8492: f64, t481: f64, t986: f64, t795: f64, t113: f64, t5086: f64, t104: f64, t494: f64, t1275: f64, t502: f64, t1277: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31929 = t352 * t8492;
    let t32094 = t986 * t481;
    let t32212 = t986 * t795;
    let t36967 = t113 * t5086;
    let t36985 = t104 * t494;
    let t37028 = t502 * t1275;
    let t37029 = t37028 * t1277;
    (t31929, t32094, t32212, t36967, t36985, t37028, t37029)
}
