//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 889/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk889<F: Float>(t825: F, t9847: F, t1114: F, t3047: F, t3083: F, t3052: F, t2848: F, t823: F, t2362: F, t2373: F, t2379: F, t3055: F, t3077: F, t3913: F, t3917: F, t3921: F, t827: F, t833: F, t8598: F, t8611: F, t9815: F, t9820: F, t9827: F, t9832: F, t9838: F) -> (F, F, F) {
    let t9848 = t9847 * t825;
    let t9849 = t1114 * t9848;
    let t9852 = t3083 * t3047;
    let t9854 = t3083 * t3052;
    let t9856 = t823 * t2848;
    let t9857 = t9856 * t825;
    let t9858 = t1114 * t9857;
    let t9861 = -t3917 * t2373 / F::cast_from(48.0_f64) - t3917 * t2379 / F::cast_from(96.0_f64) - t9815 * t2362 / F::cast_from(96.0_f64) + t827 * t9820 / F::cast_from(16.0_f64) - t3913 * t2379 / F::cast_from(96.0_f64) - t3055 * t9827 / F::cast_from(96.0_f64) - t3055 * t9832 / F::cast_from(96.0_f64) - t3913 * t2373 / F::cast_from(48.0_f64) + t3077 * t9838 / F::cast_from(48.0_f64) - t3055 * t8611 / F::cast_from(48.0_f64) - t3921 * t2373 / F::cast_from(48.0_f64) - t3921 * t2379 / F::cast_from(96.0_f64) - t9849 * t2362 / F::cast_from(96.0_f64) + t8598 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t9852 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t9854 + t9858 * t833 / F::cast_from(96.0_f64);
    (t9856, t9858, t9861)
}
