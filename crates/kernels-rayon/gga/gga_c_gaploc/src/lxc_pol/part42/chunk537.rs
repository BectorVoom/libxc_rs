//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 537/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk537(t10409: f64, t9263: f64, t9422: f64, t9442: f64, t9446: f64, t9451: f64, t1415: f64, t2897: f64, t7030: f64, t544: f64, t8237: f64, t9287: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10410 = t9263 * t10409;
    let t10411 = 0.38342925953920749676e0_f64 * t10410;
    let t10412 = 0.63904876589867916128e-1_f64 * t9422;
    let t10414 = 0.15976219147466979032e-1_f64 * t9442;
    let t10415 = 0.31952438294933958064e-1_f64 * t9446;
    let t10416 = 0.31952438294933958064e-1_f64 * t9451;
    let t10421 = t1415 * t2897;
    let t10422 = t10421 * t7030;
    let t10423 = 0.14896037479937677779e-1_f64 * t10422;
    let t10424 = t544 * t8237;
    let t10425 = t10424 * t9287;
    (t10410, t10411, t10412, t10414, t10415, t10416, t10422, t10423, t10425)
}
