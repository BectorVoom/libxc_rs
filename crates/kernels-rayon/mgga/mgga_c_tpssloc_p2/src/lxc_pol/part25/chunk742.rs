//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 742/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk742(t812: f64, t9666: f64, t2635: f64, t2690: f64, t815: f64, t831: f64, t2617: f64, t2638: f64, t2639: f64, t2681: f64, t184: f64, t2250: f64) -> (f64, f64, f64, f64, f64) {
    let t9667 = t812 * t9666;
    let t9668 = t9667 * t2635;
    let t9670 = t815 * t2690;
    let t9671 = t812 * t9670;
    let t9672 = t9671 * t831;
    let t9674 = t2617 * t2638;
    let t9675 = t9674 * t831;
    let t9679 = t2639 * t2681;
    let t9681 = t184 * t2250;
    (t9668, t9672, t9675, t9679, t9681)
}
