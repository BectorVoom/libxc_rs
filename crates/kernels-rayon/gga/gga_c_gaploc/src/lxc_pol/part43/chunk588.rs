//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 588/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk588(t7980: f64, t874: f64, t1445: f64, t574: f64, t2293: f64, t2778: f64, t1580: f64, t3399: f64, t10140: f64, t597: f64, t10144: f64, t10241: f64, t4130: f64, t590: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10370 = t7980 * t874;
    let t10371 = t1445 * t10370;
    let t10373 = 0.46011511144704899612e1_f64 * t574 * t10371;
    let t10374 = t2778 * t2293;
    let t10375 = t1445 * t10374;
    let t10377 = 0.46011511144704899612e1_f64 * t574 * t10375;
    let t10381 = 0.11502877786176224903e2_f64 * t1580 * t3399;
    let t10382 = t1445 * t10140;
    let t10384 = 0.11502877786176224903e2_f64 * t597 * t10382;
    let t10385 = t1445 * t10144;
    let t10387 = 0.11502877786176224903e2_f64 * t597 * t10385;
    let t10392 = t4130 * t10241 * t590;
    (t10373, t10377, t10381, t10384, t10387, t10392)
}
