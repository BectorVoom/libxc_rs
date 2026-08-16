//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1435/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1435(t1671: f64, t71877: f64, t18686: f64, t6021: f64, t6024: f64, t63755: f64, t21810: f64, t4740: f64, t21813: f64, t51120: f64, t1164: f64, t6088: f64, t64537: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t78327 = 4.0_f64 * t71877 * t1671;
    let t78329 = 6.0_f64 * t18686 * t6021;
    let t78331 = 0.96491876992155210402e2_f64 * t63755 * t6024;
    let t78333 = 4.0_f64 * t4740 * t21810;
    let t78335 = 0.2069040516770936012e4_f64 * t51120 * t21813;
    let t78338 = 0.62337092780453269531e3_f64 * t1164 * t64537 * t6088;
    (t78327, t78329, t78331, t78333, t78335, t78338)
}
