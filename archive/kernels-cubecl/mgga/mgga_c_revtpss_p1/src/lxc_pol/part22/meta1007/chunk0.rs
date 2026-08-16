//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3443/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3443<F: Float>(t3056: F, t6234: F, t378: F, t1076: F, t11121: F, t11195: F, t16275: F, t16318: F, t16328: F, t19342: F, t20175: F, t20211: F, t3047: F, t3058: F, t3059: F, t3060: F, t3076: F, t3269: F, t3325: F, t3326: F, t4747: F, t4752: F, t4758: F, t53160: F, t53167: F, t55413: F, t6350: F, t6393: F) -> (F, F) {
    let t64686 = t6234 * t3056;
    let t64687 = t64686 * t378;
    let t64694 = -F::cast_from(0.65854491829355115987e0_f64) * t20211 * t3076 + F::cast_from(0.52683593463484092788e1_f64) * t55413 * t4758 + F::cast_from(0.26341796731742046394e1_f64) * t3058 * t3269 * t6350 * t3059 - F::cast_from(0.13170898365871023197e1_f64) * t20175 * t3326 - F::cast_from(0.65854491829355115987e0_f64) * t11195 * t6393 + F::cast_from(0.26341796731742046394e1_f64) * t4752 * t16318 + F::cast_from(0.52683593463484092788e1_f64) * t53167 * t4758 - F::cast_from(0.79025390195226139182e1_f64) * t53160 * t16275 - F::cast_from(0.26341796731742046394e1_f64) * t3047 * t19342 + F::cast_from(0.26341796731742046394e1_f64) * t4747 * t16328 + F::cast_from(0.13170898365871023197e1_f64) * t64687 * t3060 - F::cast_from(0.39512695097613069591e1_f64) * t1076 * t11121 * t6350 * t3325;
    (t64686, t64694)
}
