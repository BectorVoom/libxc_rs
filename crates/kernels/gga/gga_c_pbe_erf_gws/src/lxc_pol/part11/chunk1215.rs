//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1215/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1215<F: Float>(t12069: F, t13353: F, t3134: F, t45750: F, t3128: F, t44215: F, t2080: F, t339: F, t3776: F, t3803: F, t860: F, t13534: F, t3786: F, t850: F) -> (F, F, F, F, F) {
    let t49281 = t13353 * t12069 / F::new(4.0);
    let t49283 = t45750 * t3134 / F::new(24.0);
    let t49285 = F::new(3.0) / F::new(4.0) * t3128 * t44215;
    let t49295 = t2080 * t3803 * t3776 * t339 * t860 / F::new(32.0);
    let t49299 = t850 * t13534 * t3786 * t860 / F::new(32.0);
    (t49281, t49283, t49285, t49295, t49299)
}
