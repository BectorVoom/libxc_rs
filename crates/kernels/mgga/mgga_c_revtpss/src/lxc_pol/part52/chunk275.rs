//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 275/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk275<F: Float>(t1248: F, t1287: F, t487: F, t1269: F, t489: F, t1204: F, t1234: F, t1281: F, t1285: F, t460: F, t490: F, t1277: F, t1210: F, t1215: F, t1271: F, t1274: F, t495: F) -> (F, F, F, F, F) {
    let t1288 = t487 * t1248 * t1287;
    let t1291 = t489 * t1269;
    let t1294 = 0.65854491829355115987e0 * t1204 * t490 - 0.65854491829355115987e0 * t1234 * t1281 + 0.65854491829355115987e0 * t1285 * t1288 + 0.65854491829355115987e0 * t460 * t1291;
    let t1295 = t1277 * t1294;
    let t1298 = 0.65854491829355115987e0 * t1204 * t495 - 0.65854491829355115987e0 * t1210 * t1215 + 0.65854491829355115987e0 * t460 * t1271 - 0.65854491829355115987e0 * t1274 * t1295;
    (t1288, t1291, t1294, t1295, t1298)
}
