//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3447/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3447(t19855: f64, t993: f64, t378: f64, t1000: f64, t11190: f64, t16305: f64, t16340: f64, t16362: f64, t16374: f64, t19425: f64, t20175: f64, t20195: f64, t20211: f64, t3058: f64, t3067: f64, t3264: f64, t3269: f64, t3270: f64, t3271: f64, t4773: f64, t5016: f64, t6244: f64, t6258: f64, t6259: f64, t995: f64) -> (f64, f64) {
    let t64816 = t19855 * t993;
    let t64817 = t64816 * t378;
    let t64822 = -0.26341796731742046394e1_f64 * t16340 * t5016 - 0.13170898365871023197e1_f64 * t995 * t3269 * t6258 * t3270 - 0.26341796731742046394e1_f64 * t16374 * t4773 - 0.79025390195226139182e1_f64 * t3264 * t19425 - 0.26341796731742046394e1_f64 * t16305 * t4773 + 0.26341796731742046394e1_f64 * t3058 * t3269 * t6244 * t3270 + 0.52683593463484092788e1_f64 * t3264 * t20195 + 0.26341796731742046394e1_f64 * t20175 * t3271 - 0.26341796731742046394e1_f64 * t16362 * t5016 + 0.13170898365871023197e1_f64 * t20211 * t3067 - 0.13170898365871023197e1_f64 * t64817 * t1000 - 0.65854491829355115987e0_f64 * t11190 * t6259;
    (t64816, t64822)
}
