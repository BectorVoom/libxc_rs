//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 385/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk385(t1632: f64, t495: f64, t551: f64, t574: f64, t1398: f64, t239: f64, t5: f64, t378: f64, t753: f64, t621: f64) -> (f64, f64, f64, f64, f64) {
    let t1634 = t551 * t1632 * t495;
    let t1635 = t574 * t1634;
    let t1643 = 20.0_f64 / 9.0_f64 * t5 * t1398 * t239;
    let t1645 = t5 * t378 * t753;
    let t1647 = t621 * t621;
    (t1634, t1635, t1643, t1645, t1647)
}
