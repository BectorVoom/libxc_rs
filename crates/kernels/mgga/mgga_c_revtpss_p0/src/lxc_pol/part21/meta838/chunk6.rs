//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3145/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3145<F: Float>(t12472: F, t5142: F, t17150: F, t3523: F, t1187: F, t12430: F, t12464: F, t12470: F, t12481: F, t12491: F, t12497: F, t12501: F, t16958: F, t16979: F, t16985: F, t16989: F, t17097: F, t17151: F, t17154: F, t1744: F, t3453: F, t3471: F, t3477: F, t3496: F, t3521: F, t45061: F, t45064: F, t45157: F, t45159: F, t45168: F, t5146: F, t5163: F, t5185: F, t57802: F, t57805: F) -> F {
    let t57972 = t5142 * t12472;
    let t58000 = t17150 * t3523;
    let t58004 = F::cast_from(0.96491876992155210402e2_f64) * t3477 * t16958 * t3471 + F::cast_from(0.6207121550312808036e4_f64) * t12470 * t57972 * t3453 + F::cast_from(0.32163958997385070134e2_f64) * t3477 * t5146 * t12464 + F::cast_from(0.19964560303604640732e6_f64) * t45157 * t1744 * t45159 * t12430 - F::cast_from(0.31168546390226634765e3_f64) * t45064 * t16985 - F::cast_from(0.35089341735807877242e1_f64) * t17154 * t12497 + F::cast_from(0.51947577317044391277e2_f64) * t17097 * t12501 - F::cast_from(0.35089341735807877242e1_f64) * t45061 * t5163 + F::cast_from(0.51947577317044391277e2_f64) * t45168 * t5185 - F::cast_from(0.70178683471615754484e1_f64) * t12491 * t16979 + F::cast_from(0.10389515463408878255e3_f64) * t12481 * t16989 - F::cast_from(0.35089341735807877242e1_f64) * t3496 * t17151 * t1187 + F::cast_from(0.51947577317044391277e2_f64) * t3521 * t58000 * t1187 + t57802 + t57805;
    t58004
}
