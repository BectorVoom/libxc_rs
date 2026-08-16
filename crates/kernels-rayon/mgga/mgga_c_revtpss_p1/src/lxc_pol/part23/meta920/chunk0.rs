//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2969/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2969(t19147: f64, t4719: f64, t23694: f64, t2986: f64, t974: f64, t981: f64, t77863: f64, t964: f64, t973: f64, t19468: f64, t19134: f64, t78094: f64, t78096: f64, t78099: f64, t78154: f64, t78192: f64, t78195: f64, t78201: f64, t78203: f64, t78206: f64, t78246: f64, t78248: f64, t78251: f64, t78254: f64, t78472: f64, t78474: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t78686 = 0.35089341735807877242e1_f64 * t4719 * t19147;
    let t78690 = 0.11696447245269292414e1_f64 * t981 * t2986 * t23694 * t974;
    let t78694 = 0.5848223622634646207e0_f64 * t981 * t964 * t77863 * t973;
    let t78696 = 0.51947577317044391276e2_f64 * t4719 * t19468;
    let t78698 = 0.31168546390226634765e3_f64 * t4719 * t19134;
    let t78699 = -t78472 - t78474 + t78094 + t78096 + t78099 - t78154 + t78686 + t78690 - t78694 - t78696 + t78698 - t78192 - t78195 - t78201 + t78203 + t78206 + t78246 - t78248 - t78251 + t78254;
    (t78686, t78690, t78694, t78696, t78698, t78699)
}
