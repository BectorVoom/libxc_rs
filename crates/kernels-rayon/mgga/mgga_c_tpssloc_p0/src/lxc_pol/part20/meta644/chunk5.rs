//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2363/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2363(t3121: f64, t607: f64, t1022: f64, t4649: f64, t41666: f64, t43398: f64, t1409: f64, t9288: f64) -> (f64, f64, f64, f64) {
    let t48472 = t3121 * t607;
    let t48477 = t4649 * t1022;
    let t48496 = t43398 * t41666;
    let t48497 = t1409 * t9288;
    (t48472, t48477, t48496, t48497)
}
