//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1926/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1926(t343: f64, t4540: f64, t6734: f64, t4571: f64, t6765: f64, t4630: f64, t6755: f64, t1611: f64, t6758: f64, t1036: f64, t7586: f64, t1409: f64, t1933: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25608 = t4540 * t343;
    let t25609 = t25608 * t6734;
    let t25616 = t6765 * t4571;
    let t25618 = t6755 * t4630;
    let t25622 = t1611 * t6758;
    let t25625 = t7586 * t1036;
    let t25628 = t1933 * t1409;
    (t25608, t25609, t25616, t25618, t25622, t25625, t25628)
}
