//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1126/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1126(t11670: f64, t2124: f64, t24454: f64, t25183: f64, t10698: f64, t11702: f64, t25192: f64, t3295: f64, t10792: f64, t2201: f64, t3613: f64, t10760: f64, t22790: f64, t25577: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39506 = t11670 * t2124 * t24454;
    let t39509 = t11670 * t2124 * t25183;
    let t39511 = t10698 * t11702;
    let t39512 = 0.12805040077930161442e0_f64 * t39511;
    let t39514 = t3295 * t2124 * t25192;
    let t39517 = t2201 * t3613 * t10792;
    let t39520 = t22790 * t10760 * t25577;
    (t39506, t39509, t39512, t39514, t39517, t39520)
}
