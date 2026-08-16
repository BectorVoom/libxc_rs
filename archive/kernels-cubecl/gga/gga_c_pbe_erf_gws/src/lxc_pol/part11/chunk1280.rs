//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1280/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1280<F: Float>(t1076: F, t328: F, t1115: F, t1144: F, t12111: F, t13174: F, t13227: F, t13613: F, t2118: F, t27077: F, t27079: F, t3052: F, t3207: F, t335: F, t338: F, t3733: F, t3912: F, t3913: F, t43375: F, t43740: F, t44063: F, t44158: F, t46635: F, t46637: F, t8713: F, t9283: F, t9838: F) -> (F, F) {
    let t50539 = t1076 * t328;
    let t50544 = F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t44158 - F::cast_from(455.0_f64) / F::cast_from(324.0_f64) * t27077 - F::cast_from(455.0_f64) / F::cast_from(324.0_f64) * t27079 - t13174 * t3052 / F::cast_from(12.0_f64) - t43740 * t3733 / F::cast_from(16.0_f64) + t3913 * t12111 / F::cast_from(8.0_f64) - t1115 * t44063 / F::cast_from(4.0_f64) - t1115 * t43375 / F::cast_from(8.0_f64) - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t3207 * t9283 * t8713 * t13227 - t335 * t338 * t1144 * t13613 / F::cast_from(4.0_f64) + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t46635 + F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t46637 + t3912 * t2118 * t50539 * t9838 / F::cast_from(8.0_f64);
    (t50539, t50544)
}
