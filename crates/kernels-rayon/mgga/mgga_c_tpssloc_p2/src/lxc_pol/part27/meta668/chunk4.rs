//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2359/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2359(t22607: f64, t7754: f64, t6875: f64, t8944: f64, t26164: f64, t1983: f64, t22578: f64, t7753: f64, t7756: f64, t531: f64, t7752: f64, t22596: f64) -> (f64, f64, f64, f64, f64) {
    let t91666 = t22607 * t7754;
    let t91669 = t6875 * t8944;
    let t91671 = 4.0_f64 * t91669 * t26164;
    let t91673 = t1983 * t7753 * t22578;
    let t91674 = t22607 * t7756;
    let t91675 = t531 * t7752;
    let t91678 = 6.0_f64 * t1983 * t91675 * t22596;
    (t91666, t91671, t91673, t91674, t91678)
}
