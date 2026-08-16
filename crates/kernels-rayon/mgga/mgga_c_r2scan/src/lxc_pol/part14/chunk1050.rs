//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1050/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1050(t10641: f64, t1102: f64, t3314: f64, t1615: f64, t2317: f64, t269: f64, t3438: f64, t6855: f64, t874: f64, t10935: f64, t2068: f64, t3446: f64) -> (f64, f64, f64) {
    let t37380 = t1102 * t3314 * t10641;
    let t37386 = t6855 * t1615 * t2317 * t3438 * t269 * t874;
    let t37390 = t3446 * t10935 * t2068;
    (t37380, t37386, t37390)
}
