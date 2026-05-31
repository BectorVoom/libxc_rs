//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1197/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1197<F: Float>(t1477: F, t2153: F, t863: F, t2160: F, t328: F, t6552: F, t331: F, t20934: F, t858: F, t867: F, t21287: F, t6240: F) -> (F, F, F) {
    let t21293 = t863 * t2153 * t1477;
    let t21294 = t21293 * t2160;
    let t21295 = F::cast_from(35.0_f64) / F::cast_from(36.0_f64) * t21294;
    let t21296 = t6552 * t328;
    let t21298 = t863 * t21296 * t331;
    let t21302 = t21298 * t867 * t858 * t20934 / F::cast_from(4.0_f64);
    let t21306 = F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t6240 * t867 * t858 * t21287;
    (t21295, t21302, t21306)
}
