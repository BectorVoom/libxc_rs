//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 991/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk991(t24615: f64, t5059: f64, t7300: f64, t5088: f64, t7301: f64, t2144: f64, t4940: f64, t1238: f64, t24575: f64, t24577: f64, t24587: f64, t27383: f64, t27389: f64, t27392: f64, t27396: f64, t27401: f64, t27403: f64, t27406: f64, t3593: f64, t498: f64, t7283: f64, t7291: f64, t7303: f64, t8061: f64) -> (f64, f64, f64, f64) {
    let t27411 = t24615 * t5059;
    let t27412 = t7300 * t27411;
    let t27415 = t7301 * t5088;
    let t27416 = t7300 * t27415;
    let t27419 = t4940 * t2144;
    let t27421 = 0.82246703342411321825e-2_f64 * t7283 * t27383 - 0.27415567780803773942e-2_f64 * t24575 - 0.27415567780803773942e-2_f64 * t24577 - 0.27415567780803773942e-2_f64 * t7283 * t27389 + 0.82246703342411321825e-2_f64 * t7283 * t27392 - t24587 + 2.0_f64 * t1238 * t27396 + 2.0_f64 * t3593 * t8061 - 0.91385225936012579807e-3_f64 * t27401 - 0.82246703342411321825e-2_f64 * t7283 * t27403 + 0.21932454224643019153e-1_f64 * t27406 * t7303 + 0.21932454224643019153e-1_f64 * t27406 * t7291 + 0.16449340668482264365e-1_f64 * t7283 * t27412 - 0.82246703342411321825e-2_f64 * t7283 * t27416 + t27419 * t498;
    (t27411, t27415, t27419, t27421)
}
