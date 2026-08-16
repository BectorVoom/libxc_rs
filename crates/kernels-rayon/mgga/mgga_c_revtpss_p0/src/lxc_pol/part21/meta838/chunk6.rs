//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3145/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3145(t12472: f64, t5142: f64, t17150: f64, t3523: f64, t1187: f64, t12430: f64, t12464: f64, t12470: f64, t12481: f64, t12491: f64, t12497: f64, t12501: f64, t16958: f64, t16979: f64, t16985: f64, t16989: f64, t17097: f64, t17151: f64, t17154: f64, t1744: f64, t3453: f64, t3471: f64, t3477: f64, t3496: f64, t3521: f64, t45061: f64, t45064: f64, t45157: f64, t45159: f64, t45168: f64, t5146: f64, t5163: f64, t5185: f64, t57802: f64, t57805: f64) -> f64 {
    let t57972 = t5142 * t12472;
    let t58000 = t17150 * t3523;
    let t58004 = 0.96491876992155210402e2_f64 * t3477 * t16958 * t3471 + 0.6207121550312808036e4_f64 * t12470 * t57972 * t3453 + 0.32163958997385070134e2_f64 * t3477 * t5146 * t12464 + 0.19964560303604640732e6_f64 * t45157 * t1744 * t45159 * t12430 - 0.31168546390226634765e3_f64 * t45064 * t16985 - 0.35089341735807877242e1_f64 * t17154 * t12497 + 0.51947577317044391277e2_f64 * t17097 * t12501 - 0.35089341735807877242e1_f64 * t45061 * t5163 + 0.51947577317044391277e2_f64 * t45168 * t5185 - 0.70178683471615754484e1_f64 * t12491 * t16979 + 0.10389515463408878255e3_f64 * t12481 * t16989 - 0.35089341735807877242e1_f64 * t3496 * t17151 * t1187 + 0.51947577317044391277e2_f64 * t3521 * t58000 * t1187 + t57802 + t57805;
    t58004
}
