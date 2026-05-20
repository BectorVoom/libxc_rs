//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3439/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3439<F: Float>(t379: F, t4746: F, t1679: F, t3057: F, t1078: F, t6244: F, t1079: F, t11214: F, t11224: F, t15578: F, t16254: F, t16305: F, t16312: F, t16313: F, t16314: F, t16603: F, t16605: F, t19400: F, t20204: F, t3066: F, t3076: F, t3268: F, t3325: F, t4764: F, t4941: F, t53108: F, t53174: F, t56087: F, t6258: F, t6259: F, t6392: F, t995: F) -> F {
    let t64547 = t4746 * t379;
    let t64550 = t3057 * t1679;
    let t64555 = t1078 * t6244;
    let t64567 = F::cast_from(0.65854491829355115987e0_f64) * t995 * t1079 * t6258 * t3325 + F::cast_from(0.52683593463484092788e1_f64) * t11224 * t19400 - F::cast_from(0.65854491829355115987e0_f64) * t20204 * t3076 - F::cast_from(0.65854491829355115987e0_f64) * t11214 * t6259 - F::cast_from(0.26341796731742046394e1_f64) * t16603 * t3268 * t6392 * t3066 - F::cast_from(0.52683593463484092788e1_f64) * t64547 * t16605 - F::cast_from(0.52683593463484092788e1_f64) * t64550 * t16314 + F::cast_from(0.26341796731742046394e1_f64) * t16305 * t4941 + F::cast_from(0.79025390195226139182e1_f64) * t53174 * t64555 * t3066 + F::cast_from(0.26341796731742046394e1_f64) * t16305 * t4764 - F::cast_from(0.26341796731742046394e1_f64) * t16312 * t16313 * t15578 - F::cast_from(0.15805078039045227836e2_f64) * t56087 * t53108 * t16254;
    t64567
}
