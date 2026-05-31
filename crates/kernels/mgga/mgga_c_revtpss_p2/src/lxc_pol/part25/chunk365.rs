//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 365/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk365<F: Float>(t265: F, t502: F, t1277: F, t1294: F, t1204: F, t1210: F, t1215: F, t1271: F, t1274: F, t460: F, t495: F, t498: F, t1128: F, t1153: F, t1193: F, t1195: F, t1200: F, t198: F, t336: F, t895: F) -> (F, F, F, F) {
    let t503 = t265 < t502;
    let t1295 = t1277 * t1294;
    let t1298 = F::cast_from(0.65854491829355115987e0_f64) * t1204 * t495 - F::cast_from(0.65854491829355115987e0_f64) * t1210 * t1215 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t1271 - F::cast_from(0.65854491829355115987e0_f64) * t1274 * t1295;
    let t1300 = F::cast_from(1.0_f64) / t498;
    let t1304 = piecewise3::<F>(t503, t1298 * t1300 * t198 * t336 - t1128 + t1153 + t1193 + t1195 - t1200, t895);
    (t1295, t1298, t1300, t1304)
}
