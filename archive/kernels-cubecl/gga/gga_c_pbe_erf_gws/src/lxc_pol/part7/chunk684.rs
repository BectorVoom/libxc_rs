//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 684/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk684<F: Float>(t1623: F, t5493: F, t1620: F, t1624: F, t4913: F, t256: F, t5443: F, t5445: F, t5449: F, t5452: F, t5458: F, t5460: F, t5462: F, t5466: F, t5469: F, t5472: F, t5474: F, t5476: F, t5479: F, t5483: F, t5487: F, t5492: F) -> (F, F, F, F) {
    let t5494 = t5493 * t1623;
    let t5495 = t1620 * t5494;
    let t5496 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t5495;
    let t5498 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t4913 * t1624;
    let t5499 = -t5443 + t5445 * t256 / F::cast_from(3.0_f64) + t5449 + F::cast_from(0.18233333333333333333e0_f64) * t5452 + t5458 + t5460 - t5462 - t5466 + t5469 + t5472 + t5474 + t5476 + t5479 + t5483 + t5487 + t5492 - t5496 - t5498;
    (t5494, t5496, t5498, t5499)
}
