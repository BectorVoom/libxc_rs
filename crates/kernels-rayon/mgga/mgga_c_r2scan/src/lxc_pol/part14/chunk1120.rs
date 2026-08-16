//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1120/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1120(t10698: f64, t11702: f64, t2124: f64, t25192: f64, t3295: f64, t10792: f64, t2201: f64, t3613: f64, t10760: f64, t22790: f64, t25577: f64, t10885: f64, t11744: f64) -> (f64, f64, f64, f64, f64) {
    let t39511 = t10698 * t11702;
    let t39514 = t3295 * t2124 * t25192;
    let t39517 = t2201 * t3613 * t10792;
    let t39520 = t22790 * t10760 * t25577;
    let t39522 = t11744 * t10885;
    (t39511, t39514, t39517, t39520, t39522)
}
