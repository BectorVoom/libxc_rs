//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1177/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1177(t14850: f64, t4786: f64, t1117: f64, t5989: f64, t3313: f64, t1671: f64, t4781: f64, t3264: f64, t6024: f64, t11190: f64, t1098: f64, t5983: f64) -> (f64, f64, f64, f64, f64) {
    let t18676 = 0.32163958997385070134e2_f64 * t14850 * t4786;
    let t18677 = t5989 * t1117;
    let t18679 = 6.0_f64 * t3313 * t18677;
    let t18680 = t1671 * t4781;
    let t18682 = 4.0_f64 * t3264 * t18680;
    let t18683 = t6024 * t1117;
    let t18685 = 0.96491876992155210402e2_f64 * t11190 * t18683;
    let t18686 = t5983 * t1098;
    (t18676, t18679, t18682, t18685, t18686)
}
