//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2969/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2969<F: Float>(t19147: F, t4719: F, t23694: F, t2986: F, t974: F, t981: F, t77863: F, t964: F, t973: F, t19468: F, t19134: F, t78094: F, t78096: F, t78099: F, t78154: F, t78192: F, t78195: F, t78201: F, t78203: F, t78206: F, t78246: F, t78248: F, t78251: F, t78254: F, t78472: F, t78474: F) -> (F, F, F, F, F, F) {
    let t78686 = F::cast_from(0.35089341735807877242e1_f64) * t4719 * t19147;
    let t78690 = F::cast_from(0.11696447245269292414e1_f64) * t981 * t2986 * t23694 * t974;
    let t78694 = F::cast_from(0.5848223622634646207e0_f64) * t981 * t964 * t77863 * t973;
    let t78696 = F::cast_from(0.51947577317044391276e2_f64) * t4719 * t19468;
    let t78698 = F::cast_from(0.31168546390226634765e3_f64) * t4719 * t19134;
    let t78699 = -t78472 - t78474 + t78094 + t78096 + t78099 - t78154 + t78686 + t78690 - t78694 - t78696 + t78698 - t78192 - t78195 - t78201 + t78203 + t78206 + t78246 - t78248 - t78251 + t78254;
    (t78686, t78690, t78694, t78696, t78698, t78699)
}
