//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 936/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk936<F: Float>(t133: F, t8199: F, t2911: F, t2912: F, t8152: F, t8177: F, t8181: F, t8182: F, t8186: F, t8193: F, t8198: F, t8231: F, t8232: F, t8238: F, t8240: F, t8244: F, t8249: F) -> F {
    let t8252 = t133 * t8199;
    let t8254 = t8177 - t8181 - t8182 - F::new(0.2069106e2) * t2911 * t8231 * t8232 - t8186 - F::new(0.344851e1) * t8238 + F::new(0.1034553e2) * t2911 * t2912 * t8240 + F::new(0.5172765e1) * t2911 * t2912 * t8244 - t8193 - t8198 + t8249 - F::new(0.1724255e1) * t133 * t8152 - F::cast_from(0.76633555555555555556e0_f64) * t8252;
    t8254
}
