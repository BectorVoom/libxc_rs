//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2946/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2946(t1610: f64, t19127: f64, t2874: f64, t11294: f64, t23770: f64, t1609: f64, t2924: f64, t63650: f64, t23694: f64, t3014: f64, t11461: f64, t11507: f64, t15406: f64, t1633: f64, t19279: f64, t19283: f64, t19303: f64, t19310: f64, t23451: f64, t23714: f64, t23717: f64, t23764: f64, t2987: f64, t3012: f64, t41238: f64, t41658: f64, t41759: f64, t4652: f64, t4674: f64, t4707: f64, t52825: f64, t64060: f64, t64072: f64, t64319: f64, t972: f64) -> (f64, f64, f64, f64) {
    let t78201 = 6.0_f64 * t2874 * t1610 * t19127;
    let t78203 = 0.48245938496077605201e2_f64 * t11294 * t23770;
    let t78206 = 0.48245938496077605201e2_f64 * t2924 * t63650 * t1609;
    let t78207 = t23694 * t3014;
    let t78240 = t78201 - t78203 - t78206 + 0.17315859105681463759e2_f64 * t3012 * t78207 * t972 - 0.12304822629859687989e5_f64 * t41759 * t23717 * t972 + 0.30762056574649219974e4_f64 * t11507 * t19310 * t4707 + 0.91082604192152556044e5_f64 * t41658 * t23451 * t41238 * t972 + 0.1929837539843104208e3_f64 * t15406 * t19279 + 0.62071215503128080361e4_f64 * t52825 * t19283 + 0.51947577317044391277e2_f64 * t11461 * t23764 + 0.51947577317044391277e2_f64 * t3012 * t64072 * t1633 + 0.51947577317044391277e2_f64 * t3012 * t19303 * t4707 - 0.11696447245269292414e1_f64 * t2987 * t23714 * t972 - 6.0_f64 * t64319 * t4652 + 0.96491876992155210402e2_f64 * t64060 * t4674;
    (t78201, t78203, t78206, t78240)
}
