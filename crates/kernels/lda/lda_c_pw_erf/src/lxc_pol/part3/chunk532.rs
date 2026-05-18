//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 532/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk532<F: Float>(t159: F, t1904: F, t285: F, t477: F, t780: F, t281: F, t684: F, t872: F, t1096: F, t1158: F, t1161: F, t1165: F, t1176: F, t1181: F, t1189: F, t1195: F, t1740: F, t2237: F, t2303: F, t279: F, t296: F) -> (F, F, F) {
    let t2306 = t1904 * t159 * t285;
    let t2310 = t780 * t477 * t285;
    let t2311 = t281 * t2310;
    let t2313 = t684 * t872;
    let t2315 = -F::new(0.01197423401025461) * t1176 - t1181 - t1189 - t1740 + t1158 - F::new(0.0002905674151788692) * t1161 - t1165 + t1195 - F::new(0.054045904796391424) * t1096 + t2237 * t296 + t2303 * t279 - F::new(0.01197423401025461) * t281 * t2306 - F::new(0.01197423401025461) * t2311 + F::new(0.019957056683757683) * t2313;
    (t2306, t2310, t2315)
}
