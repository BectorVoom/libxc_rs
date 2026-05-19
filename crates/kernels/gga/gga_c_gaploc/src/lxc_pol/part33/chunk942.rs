//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 942/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk942<F: Float>(t2478: F, t993: F, t6576: F, t2890: F, t6583: F, t2482: F, t9263: F, t9422: F, t9442: F, t9446: F, t9451: F, t1415: F, t2897: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10399 = t993 * t2478;
    let t10400 = t6576 * t10399;
    let t10401 = F::cast_from(0.19171462976960374838e0_f64) * t10400;
    let t10402 = t2890 * t2478;
    let t10403 = t6583 * t10402;
    let t10404 = F::cast_from(0.19171462976960374838e0_f64) * t10403;
    let t10409 = t993 * t2482;
    let t10410 = t9263 * t10409;
    let t10411 = F::cast_from(0.38342925953920749676e0_f64) * t10410;
    let t10412 = F::cast_from(0.63904876589867916128e-1_f64) * t9422;
    let t10414 = F::cast_from(0.15976219147466979032e-1_f64) * t9442;
    let t10415 = F::cast_from(0.31952438294933958064e-1_f64) * t9446;
    let t10416 = F::cast_from(0.31952438294933958064e-1_f64) * t9451;
    let t10421 = t1415 * t2897;
    (t10399, t10401, t10402, t10404, t10409, t10411, t10412, t10414, t10415, t10416, t10421)
}
