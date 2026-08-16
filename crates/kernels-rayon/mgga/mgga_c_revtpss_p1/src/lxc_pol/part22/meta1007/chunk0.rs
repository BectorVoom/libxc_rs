//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3443/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3443(t3056: f64, t6234: f64, t378: f64, t1076: f64, t11121: f64, t11195: f64, t16275: f64, t16318: f64, t16328: f64, t19342: f64, t20175: f64, t20211: f64, t3047: f64, t3058: f64, t3059: f64, t3060: f64, t3076: f64, t3269: f64, t3325: f64, t3326: f64, t4747: f64, t4752: f64, t4758: f64, t53160: f64, t53167: f64, t55413: f64, t6350: f64, t6393: f64) -> (f64, f64) {
    let t64686 = t6234 * t3056;
    let t64687 = t64686 * t378;
    let t64694 = -0.65854491829355115987e0_f64 * t20211 * t3076 + 0.52683593463484092788e1_f64 * t55413 * t4758 + 0.26341796731742046394e1_f64 * t3058 * t3269 * t6350 * t3059 - 0.13170898365871023197e1_f64 * t20175 * t3326 - 0.65854491829355115987e0_f64 * t11195 * t6393 + 0.26341796731742046394e1_f64 * t4752 * t16318 + 0.52683593463484092788e1_f64 * t53167 * t4758 - 0.79025390195226139182e1_f64 * t53160 * t16275 - 0.26341796731742046394e1_f64 * t3047 * t19342 + 0.26341796731742046394e1_f64 * t4747 * t16328 + 0.13170898365871023197e1_f64 * t64687 * t3060 - 0.39512695097613069591e1_f64 * t1076 * t11121 * t6350 * t3325;
    (t64686, t64694)
}
