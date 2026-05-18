//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 537/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk537<F: Float>(t10409: F, t9263: F, t9422: F, t9442: F, t9446: F, t9451: F, t1415: F, t2897: F, t7030: F, t544: F, t8237: F, t9287: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10410 = t9263 * t10409;
    let t10411 = F::new(0.38342925953920749676e0) * t10410;
    let t10412 = F::new(0.63904876589867916128e-1) * t9422;
    let t10414 = F::new(0.15976219147466979032e-1) * t9442;
    let t10415 = F::new(0.31952438294933958064e-1) * t9446;
    let t10416 = F::new(0.31952438294933958064e-1) * t9451;
    let t10421 = t1415 * t2897;
    let t10422 = t10421 * t7030;
    let t10423 = F::new(0.14896037479937677779e-1) * t10422;
    let t10424 = t544 * t8237;
    let t10425 = t10424 * t9287;
    (t10410, t10411, t10412, t10414, t10415, t10416, t10422, t10423, t10425)
}
