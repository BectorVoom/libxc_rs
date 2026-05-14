//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 551/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk551<F: Float>(t10399: F, t6576: F, t2478: F, t2890: F, t6583: F, t1339: F, t3338: F, t590: F, t2482: F, t993: F, t9263: F, t9422: F, t10381: F, t10384: F, t10387: F, t10388: F, t10394: F, t10395: F, t10398: F, t1537: F, t567: F, t9363: F, t9366: F, t9370: F) -> (F, F) {
    let t10400 = t6576 * t10399;
    let t10401 = 0.19171462976960374838e0 * t10400;
    let t10402 = t2890 * t2478;
    let t10403 = t6583 * t10402;
    let t10404 = 0.19171462976960374838e0 * t10403;
    let t10405 = t1339 * t3338;
    let t10406 = t10405 * t590;
    let t10409 = t993 * t2482;
    let t10410 = t9263 * t10409;
    let t10411 = 0.38342925953920749676e0 * t10410;
    let t10412 = 0.63904876589867916128e-1 * t9422;
    let t10413 = t10381 + t10384 + t10387 + 0.23005755572352449806e1 * t567 * t10388 + t9363 + t9366 - t9370 + t10394 - t10395 - t10398 + t10401 - t10404 - 0.51123901271894332902e0 * t1537 * t10406 - t10411 + t10412;
    (t10405, t10413)
}
