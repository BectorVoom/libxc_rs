//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 521/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk521<F: Float>(t2182: F, t858: F, t2210: F, t884: F, t2074: F, t886: F, t2089: F, t2117: F, t2126: F, t2131: F, t2140: F, t2144: F, t2152: F, t2162: F, t2166: F, t2175: F, t2194: F, t2199: F, t2204: F, t2208: F) -> (F, F, F, F, F) {
    let t2211 = t858 * t2182;
    let t2212 = t2210 * t2211;
    let t2214 = t884 * t2212 / F::cast_from(16.0_f64);
    let t2215 = t858 * t2074;
    let t2216 = t886 * t2215;
    let t2218 = t884 * t2216 / F::cast_from(48.0_f64);
    let t2219 = t2089 + t2117 - t2126 + t2131 - t2140 - t2144 - t2152 + t2162 + t2166 + t2175 - t2194 - t2199 + t2204 + t2208 + t2214 - t2218;
    (t2212, t2214, t2216, t2218, t2219)
}
