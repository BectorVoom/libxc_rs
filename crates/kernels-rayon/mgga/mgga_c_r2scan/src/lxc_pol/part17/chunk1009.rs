//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1009/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1009(t12492: f64, t6085: f64, t10760: f64, t9246: f64, t6093: f64, t261: f64, t3217: f64, t3299: f64, t11748: f64, t3594: f64, t3223: f64, t3304: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12493 = t6085 * t12492;
    let t12495 = t10760 * t9246;
    let t12496 = t6093 * t12495;
    let t12498 = t261 * t3217;
    let t12499 = t3299 * t12498;
    let t12501 = t11748 * t3594;
    let t12503 = t261 * t3223;
    let t12504 = t3304 * t12503;
    (t12493, t12495, t12496, t12498, t12499, t12501, t12503, t12504)
}
