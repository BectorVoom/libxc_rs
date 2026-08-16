//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1220/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1220(t29473: f64, t67: f64, t1864: f64, t7445: f64, t7974: f64, t2109: f64, t27956: f64, t1860: f64, t2110: f64, t24514: f64, t26016: f64, t27298: f64, t27332: f64, t27341: f64, t27937: f64, t27961: f64, t27966: f64, t27972: f64, t27976: f64, t27979: f64, t27982: f64, t7246: f64, t7428: f64, t7432: f64, t7435: f64, t7975: f64, t7978: f64) -> (f64, f64, f64, f64, f64) {
    let t29474 = t29473 * t67;
    let t29475 = t29474 * t1864;
    let t29478 = t7974 * t7445;
    let t29481 = t2109 * t27956;
    let t29484 = -5.0_f64 * t24514 * t27961 - 10.0_f64 / 3.0_f64 * t26016 * t27298 + 5.0_f64 / 3.0_f64 * t27341 * t7432 + 2.0_f64 / 3.0_f64 * t27966 * t2110 + 5.0_f64 / 3.0_f64 * t27332 * t7432 + 5.0_f64 / 3.0_f64 * t7246 * t27972 + 5.0_f64 / 6.0_f64 * t7246 * t27976 + t27979 * t2110 / 3.0_f64 + t27982 * t2110 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t7435 * t7975 + 2.0_f64 / 3.0_f64 * t7435 * t7978 - t27937 * t2110 / 6.0_f64 - t7428 * t7975 / 3.0_f64 - t7428 * t7978 / 3.0_f64 - t1860 * t29475 / 6.0_f64 - t1860 * t29478 / 3.0_f64 - t1860 * t29481 / 6.0_f64;
    (t29474, t29475, t29478, t29481, t29484)
}
