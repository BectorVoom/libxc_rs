//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1287/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1287(t111: f64, t8153: f64, t1851: f64, t8171: f64, t110140: f64, t8223: f64, t29895: f64, t30152: f64, t110143: f64, t8226: f64, t64: f64, t91: f64, t9365: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t110240 = t8153 * t111;
    let t110489 = 2.0_f64 * t1851 * t8171;
    let t110503 = t110140 * t8223;
    let t110506 = 20.0_f64 / 9.0_f64 * t29895 * t30152;
    let t110510 = t110143 * t8226;
    let t110520 = t64 * t9365 * t91;
    (t110240, t110489, t110503, t110506, t110510, t110520)
}
