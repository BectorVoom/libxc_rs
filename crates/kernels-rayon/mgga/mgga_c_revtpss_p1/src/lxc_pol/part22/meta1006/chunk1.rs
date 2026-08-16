//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3439/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3439(t379: f64, t4746: f64, t1679: f64, t3057: f64, t1078: f64, t6244: f64, t1079: f64, t11214: f64, t11224: f64, t15578: f64, t16254: f64, t16305: f64, t16312: f64, t16313: f64, t16314: f64, t16603: f64, t16605: f64, t19400: f64, t20204: f64, t3066: f64, t3076: f64, t3268: f64, t3325: f64, t4764: f64, t4941: f64, t53108: f64, t53174: f64, t56087: f64, t6258: f64, t6259: f64, t6392: f64, t995: f64) -> f64 {
    let t64547 = t4746 * t379;
    let t64550 = t3057 * t1679;
    let t64555 = t1078 * t6244;
    let t64567 = 0.65854491829355115987e0_f64 * t995 * t1079 * t6258 * t3325 + 0.52683593463484092788e1_f64 * t11224 * t19400 - 0.65854491829355115987e0_f64 * t20204 * t3076 - 0.65854491829355115987e0_f64 * t11214 * t6259 - 0.26341796731742046394e1_f64 * t16603 * t3268 * t6392 * t3066 - 0.52683593463484092788e1_f64 * t64547 * t16605 - 0.52683593463484092788e1_f64 * t64550 * t16314 + 0.26341796731742046394e1_f64 * t16305 * t4941 + 0.79025390195226139182e1_f64 * t53174 * t64555 * t3066 + 0.26341796731742046394e1_f64 * t16305 * t4764 - 0.26341796731742046394e1_f64 * t16312 * t16313 * t15578 - 0.15805078039045227836e2_f64 * t56087 * t53108 * t16254;
    t64567
}
