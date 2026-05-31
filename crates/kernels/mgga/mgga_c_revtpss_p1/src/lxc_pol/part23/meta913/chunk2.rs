//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2946/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2946<F: Float>(t1610: F, t19127: F, t2874: F, t11294: F, t23770: F, t1609: F, t2924: F, t63650: F, t23694: F, t3014: F, t11461: F, t11507: F, t15406: F, t1633: F, t19279: F, t19283: F, t19303: F, t19310: F, t23451: F, t23714: F, t23717: F, t23764: F, t2987: F, t3012: F, t41238: F, t41658: F, t41759: F, t4652: F, t4674: F, t4707: F, t52825: F, t64060: F, t64072: F, t64319: F, t972: F) -> (F, F, F, F) {
    let t78201 = F::cast_from(6.0_f64) * t2874 * t1610 * t19127;
    let t78203 = F::cast_from(0.48245938496077605201e2_f64) * t11294 * t23770;
    let t78206 = F::cast_from(0.48245938496077605201e2_f64) * t2924 * t63650 * t1609;
    let t78207 = t23694 * t3014;
    let t78240 = t78201 - t78203 - t78206 + F::cast_from(0.17315859105681463759e2_f64) * t3012 * t78207 * t972 - F::cast_from(0.12304822629859687989e5_f64) * t41759 * t23717 * t972 + F::cast_from(0.30762056574649219974e4_f64) * t11507 * t19310 * t4707 + F::cast_from(0.91082604192152556044e5_f64) * t41658 * t23451 * t41238 * t972 + F::cast_from(0.1929837539843104208e3_f64) * t15406 * t19279 + F::cast_from(0.62071215503128080361e4_f64) * t52825 * t19283 + F::cast_from(0.51947577317044391277e2_f64) * t11461 * t23764 + F::cast_from(0.51947577317044391277e2_f64) * t3012 * t64072 * t1633 + F::cast_from(0.51947577317044391277e2_f64) * t3012 * t19303 * t4707 - F::cast_from(0.11696447245269292414e1_f64) * t2987 * t23714 * t972 - F::cast_from(6.0_f64) * t64319 * t4652 + F::cast_from(0.96491876992155210402e2_f64) * t64060 * t4674;
    (t78201, t78203, t78206, t78240)
}
