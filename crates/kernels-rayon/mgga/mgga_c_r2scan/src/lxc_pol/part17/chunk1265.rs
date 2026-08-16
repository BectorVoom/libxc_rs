//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1265/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1265(t11531: f64, t12098: f64, t3275: f64, t3262: f64, t3465: f64, t42959: f64, t11336: f64, t39263: f64, t42863: f64, t11325: f64, t12428: f64, t12951: f64, t37282: f64) -> (f64, f64, f64, f64, f64) {
    let t44882 = 5.0_f64 / 8.0_f64 * t3275 * t12098 * t11531;
    let t44885 = 3.0_f64 / 2.0_f64 * t3262 * t3465 * t42959;
    let t44888 = 3.0_f64 * t39263 * t11336 * t42863;
    let t44893 = 5.0_f64 / 16.0_f64 * t3275 * t11325 * t12428;
    let t44897 = 15.0_f64 / 8.0_f64 * t37282 * t12951;
    (t44882, t44885, t44888, t44893, t44897)
}
