//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 769/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk769<F: Float>(t12549: F, t590: F, t587: F, t2615: F, t3531: F, t12339: F, t5294: F, t5293: F, t10378: F, t995: F, t1885: F, t1820: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12550 = t590 * t12549;
    let t12552 = F::new(4.0) / F::new(45.0) * t587 * t12550;
    let t12554 = F::new(4.0) / F::new(9.0) * t2615 * t3531;
    let t12555 = t5294 * t12339;
    let t12556 = t5293 * t12555;
    let t12558 = F::new(32.0) / F::new(81.0) * t587 * t12556;
    let t12559 = t10378 * t995;
    let t12560 = t1885 * t12559;
    let t12562 = F::new(8.0) / F::new(5.0) * t1820 * t12560;
    (t12550, t12552, t12554, t12555, t12556, t12558, t12559, t12560, t12562)
}
