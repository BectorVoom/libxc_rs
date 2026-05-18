//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1272/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1272<F: Float>(t2155: F, t50118: F, t858: F, t867: F, t1105: F, t1153: F, t11994: F, t12072: F, t13593: F, t20733: F, t2255: F, t2277: F, t2312: F, t274: F, t3257: F, t3763: F, t37645: F, t44283: F, t50275: F, t50279: F, t50281: F, t50286: F, t50290: F, t50291: F, t50292: F, t6637: F, t6685: F, t9441: F, t9847: F) -> (F, F) {
    let t50299 = t2155 * t867 * t858 * t50118 / F::new(16.0);
    let t50300 = -t2312 * t2255 * t9441 * t13593 * t1105 / F::new(48.0) + F::new(7.0) / F::new(384.0) * t2277 * t3257 * t11994 * t12072 * t274 + t50275 - t50279 + t50281 - t6637 * t37645 * t9847 * t3763 / F::new(32.0) - F::new(5.0) / F::new(16.0) * t20733 * t1153 * t50286 + t50290 + t50291 + F::new(3.0) / F::new(64.0) * t6685 * t44283 * t50292 + t50299;
    (t50299, t50300)
}
