//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 941/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk941<F: Float>(t17512: F, t639: F, t1892: F, t5463: F, t1620: F, t5493: F, t5505: F, t5489: F, t4913: F, t5494: F, t17490: F, t17493: F, t17498: F, t17501: F, t17503: F, t17507: F, t17511: F) -> (F, F, F, F, F, F) {
    let t17513 = t639 * t17512;
    let t17514 = F::new(128.0) / F::new(1215.0) * t17513;
    let t17516 = t639 * t5463 * t1892;
    let t17517 = F::new(32.0) / F::new(135.0) * t17516;
    let t17519 = t1620 * t5493 * t5505;
    let t17520 = F::new(32.0) / F::new(15.0) * t17519;
    let t17522 = t639 * t5493 * t5489;
    let t17523 = F::new(32.0) / F::new(15.0) * t17522;
    let t17524 = t4913 * t5494;
    let t17525 = F::new(64.0) / F::new(15.0) * t17524;
    let t17526 = F::new(8.0) * t17490 - t17493 - t17498 - t17501 + t17503 + t17507 + t17511 + t17514 + t17517 - t17520 + t17523 - t17525;
    (t17514, t17517, t17520, t17523, t17525, t17526)
}
