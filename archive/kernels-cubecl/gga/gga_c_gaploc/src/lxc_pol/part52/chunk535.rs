//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 535/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk535<F: Float>(t7980: F, t874: F, t1445: F, t574: F, t2293: F, t2778: F, t1580: F, t3399: F, t10140: F, t597: F, t10144: F, t10241: F, t4130: F, t590: F) -> (F, F, F, F, F, F, F) {
    let t10370 = t7980 * t874;
    let t10371 = t1445 * t10370;
    let t10373 = F::cast_from(0.46011511144704899612e1_f64) * t574 * t10371;
    let t10374 = t2778 * t2293;
    let t10375 = t1445 * t10374;
    let t10377 = F::cast_from(0.46011511144704899612e1_f64) * t574 * t10375;
    let t10381 = F::cast_from(0.11502877786176224903e2_f64) * t1580 * t3399;
    let t10382 = t1445 * t10140;
    let t10384 = F::cast_from(0.11502877786176224903e2_f64) * t597 * t10382;
    let t10385 = t1445 * t10144;
    let t10387 = F::cast_from(0.11502877786176224903e2_f64) * t597 * t10385;
    let t10392 = t4130 * t10241 * t590;
    (t10370, t10373, t10377, t10381, t10384, t10387, t10392)
}
