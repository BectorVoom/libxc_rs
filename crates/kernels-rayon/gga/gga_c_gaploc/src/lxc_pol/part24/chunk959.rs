//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 959/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk959(t9442: f64, t9446: f64, t9451: f64, t1: f64, t10170: f64, t544: f64, t1415: f64, t2897: f64, t7030: f64, t8237: f64, t9287: f64, t3407: f64, t7014: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10414 = 0.15976219147466979032e-1_f64 * t9442;
    let t10415 = 0.31952438294933958064e-1_f64 * t9446;
    let t10416 = 0.31952438294933958064e-1_f64 * t9451;
    let t10417 = t10170 * t1;
    let t10418 = t544 * t10417;
    let t10421 = t1415 * t2897;
    let t10422 = t10421 * t7030;
    let t10423 = 0.14896037479937677779e-1_f64 * t10422;
    let t10424 = t544 * t8237;
    let t10425 = t10424 * t9287;
    let t10426 = 0.14896037479937677779e-1_f64 * t10425;
    let t10427 = t7014 * t3407;
    (t10414, t10415, t10416, t10417, t10418, t10421, t10423, t10424, t10426, t10427)
}
