//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 682/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk682(t4733: f64, t827: f64, t4727: f64, t5005: f64, t79: f64, t10464: f64, t26: f64, t10450: f64, t1659: f64, t10570: f64, t10572: f64, t10574: f64, t10576: f64, t10587: f64, t10595: f64, t10607: f64, t10610: f64, t10613: f64, t10615: f64) -> (f64, f64, f64, f64, f64) {
    let t10617 = t827 * t4733;
    let t10619 = t827 * t4727;
    let t10621 = t79 * t5005;
    let t10622 = t10621 * t10464;
    let t10623 = t26 * t10622;
    let t10625 = t1659 * t10450;
    let t10626 = t26 * t10625;
    let t10634 = -0.33114e0_f64 * t10607 + 0.16557e0_f64 * t10610 - 0.49671e0_f64 * t10613 - 0.27595e0_f64 * t10615 + 0.16557e0_f64 * t10617 + 0.5519e-1_f64 * t10619 - 0.36793333333333333333e-1_f64 * t10623 - 0.82785e-1_f64 * t10626 - 0.60384999999999999999e0_f64 * t10587 + 0.181155e1_f64 * t10595 - 0.40256666666666666668e0_f64 * t10570 + 0.20128333333333333333e0_f64 * t10572 - 0.60385000000000000001e0_f64 * t10574 + 0.30192500000000000001e0_f64 * t10576;
    (t10617, t10619, t10623, t10626, t10634)
}
