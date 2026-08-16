//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1431/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1431(t104735: f64, t104787: f64, t106804: f64, t2110: f64, t26016: f64, t27298: f64, t27937: f64, t27979: f64, t29475: f64, t29478: f64, t29481: f64, t7428: f64, t7975: f64, t7978: f64, t96473: f64) -> f64 {
    let t108939 = -t106804 * t2110 / 6.0_f64 - t27937 * t7975 / 2.0_f64 - t27937 * t7978 / 2.0_f64 - t7428 * t29475 / 2.0_f64 - t7428 * t29478 - t7428 * t29481 / 2.0_f64 + t27979 * t7975 + t27979 * t7978 - 5.0_f64 * t96473 * t27298 - 10.0_f64 * t26016 * t104787 - 10.0_f64 * t26016 * t104735;
    t108939
}
