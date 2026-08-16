//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3561/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3561(t1000: f64, t1076: f64, t1079: f64, t1096: f64, t11210: f64, t16312: f64, t16313: f64, t16343: f64, t1696: f64, t19380: f64, t19381: f64, t19428: f64, t20178: f64, t225: f64, t3047: f64, t3058: f64, t3271: f64, t342: f64, t385: f64, t53119: f64, t53174: f64, t6351: f64, t64831: f64, t64835: f64, t64841: f64, t64845: f64, t64896: f64, t64997: f64, t65102: f64, t65150: f64, t65196: f64, t65239: f64, t65279: f64, t67584: f64, t67633: f64, t67684: f64, t67723: f64, t67768: f64, t67813: f64, t67859: f64, t67905: f64, t67946: f64, t67989: f64, t995: f64, t996: f64) -> f64 {
    let t68006 = -0.13170898365871023197e1_f64 * t3047 * t19381 - 0.26341796731742046394e1_f64 * t53119 * t1696 + 0.13170898365871023197e1_f64 * t995 * t1079 * t19380 * t1096 + 0.26341796731742046394e1_f64 * t3058 * t996 * t64831 + 0.13170898365871023197e1_f64 * t3058 * t996 * t64835 + 0.13170898365871023197e1_f64 * t20178 * t3271 + 0.10536718692696818558e2_f64 * t16312 * t19428 * t64841 - 0.26341796731742046394e1_f64 * t64845 * t1000 - 0.65854491829355115987e0_f64 * t1076 * t1079 * (t64896 + t64997 + t65102 + t65150 + t65196 + t65239 + t65279 + t67633 + t67684 + t67723 + t67768 + t67813 + t67859 + t67905 + t67946 + t67989) + 0.65854491829355115987e0_f64 * t342 * t67584 * t225 * t385 + 0.79025390195226139182e1_f64 * t53174 * t16313 * t16343 + 0.13170898365871023197e1_f64 * t11210 * t6351;
    t68006
}
