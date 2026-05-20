//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 418/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk418<F: Float>(t1277: F, t1294: F, t1204: F, t1210: F, t1215: F, t1271: F, t1274: F, t460: F, t495: F, t498: F) -> (F, F, F) {
    let t1295 = t1277 * t1294;
    let t1298 = F::cast_from(0.65854491829355115987e0_f64) * t1204 * t495 - F::cast_from(0.65854491829355115987e0_f64) * t1210 * t1215 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t1271 - F::cast_from(0.65854491829355115987e0_f64) * t1274 * t1295;
    let t1300 = F::new(1.0) / t498;
    (t1295, t1298, t1300)
}
