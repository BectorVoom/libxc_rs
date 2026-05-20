//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3561/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3561<F: Float>(t1000: F, t1076: F, t1079: F, t1096: F, t11210: F, t16312: F, t16313: F, t16343: F, t1696: F, t19380: F, t19381: F, t19428: F, t20178: F, t225: F, t3047: F, t3058: F, t3271: F, t342: F, t385: F, t53119: F, t53174: F, t6351: F, t64831: F, t64835: F, t64841: F, t64845: F, t64896: F, t64997: F, t65102: F, t65150: F, t65196: F, t65239: F, t65279: F, t67584: F, t67633: F, t67684: F, t67723: F, t67768: F, t67813: F, t67859: F, t67905: F, t67946: F, t67989: F, t995: F, t996: F) -> F {
    let t68006 = -F::cast_from(0.13170898365871023197e1_f64) * t3047 * t19381 - F::cast_from(0.26341796731742046394e1_f64) * t53119 * t1696 + F::cast_from(0.13170898365871023197e1_f64) * t995 * t1079 * t19380 * t1096 + F::cast_from(0.26341796731742046394e1_f64) * t3058 * t996 * t64831 + F::cast_from(0.13170898365871023197e1_f64) * t3058 * t996 * t64835 + F::cast_from(0.13170898365871023197e1_f64) * t20178 * t3271 + F::cast_from(0.10536718692696818558e2_f64) * t16312 * t19428 * t64841 - F::cast_from(0.26341796731742046394e1_f64) * t64845 * t1000 - F::cast_from(0.65854491829355115987e0_f64) * t1076 * t1079 * (t64896 + t64997 + t65102 + t65150 + t65196 + t65239 + t65279 + t67633 + t67684 + t67723 + t67768 + t67813 + t67859 + t67905 + t67946 + t67989) + F::cast_from(0.65854491829355115987e0_f64) * t342 * t67584 * t225 * t385 + F::cast_from(0.79025390195226139182e1_f64) * t53174 * t16313 * t16343 + F::cast_from(0.13170898365871023197e1_f64) * t11210 * t6351;
    t68006
}
