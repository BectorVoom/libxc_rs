//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 888/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk888<F: Float>(t825: F, t9847: F, t1114: F, t3047: F, t3083: F, t3052: F, t2848: F, t823: F, t2362: F, t2373: F, t2379: F, t3055: F, t3077: F, t3913: F, t3917: F, t3921: F, t827: F, t833: F, t8598: F, t8611: F, t9815: F, t9820: F, t9827: F, t9832: F, t9838: F) -> (F, F, F) {
    let t9848 = t9847 * t825;
    let t9849 = t1114 * t9848;
    let t9852 = t3083 * t3047;
    let t9854 = t3083 * t3052;
    let t9856 = t823 * t2848;
    let t9857 = t9856 * t825;
    let t9858 = t1114 * t9857;
    let t9861 = -t3917 * t2373 / F::new(48.0) - t3917 * t2379 / F::new(96.0) - t9815 * t2362 / F::new(96.0) + t827 * t9820 / F::new(16.0) - t3913 * t2379 / F::new(96.0) - t3055 * t9827 / F::new(96.0) - t3055 * t9832 / F::new(96.0) - t3913 * t2373 / F::new(48.0) + t3077 * t9838 / F::new(48.0) - t3055 * t8611 / F::new(48.0) - t3921 * t2373 / F::new(48.0) - t3921 * t2379 / F::new(96.0) - t9849 * t2362 / F::new(96.0) + t8598 + F::new(7.0) / F::new(144.0) * t9852 + F::new(7.0) / F::new(72.0) * t9854 + t9858 * t833 / F::new(96.0);
    (t9856, t9858, t9861)
}
