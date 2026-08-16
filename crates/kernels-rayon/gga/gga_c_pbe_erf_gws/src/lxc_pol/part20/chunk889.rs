//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 889/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk889(t825: f64, t9847: f64, t1114: f64, t3047: f64, t3083: f64, t3052: f64, t2848: f64, t823: f64, t2362: f64, t2373: f64, t2379: f64, t3055: f64, t3077: f64, t3913: f64, t3917: f64, t3921: f64, t827: f64, t833: f64, t8598: f64, t8611: f64, t9815: f64, t9820: f64, t9827: f64, t9832: f64, t9838: f64) -> (f64, f64, f64) {
    let t9848 = t9847 * t825;
    let t9849 = t1114 * t9848;
    let t9852 = t3083 * t3047;
    let t9854 = t3083 * t3052;
    let t9856 = t823 * t2848;
    let t9857 = t9856 * t825;
    let t9858 = t1114 * t9857;
    let t9861 = -t3917 * t2373 / 48.0_f64 - t3917 * t2379 / 96.0_f64 - t9815 * t2362 / 96.0_f64 + t827 * t9820 / 16.0_f64 - t3913 * t2379 / 96.0_f64 - t3055 * t9827 / 96.0_f64 - t3055 * t9832 / 96.0_f64 - t3913 * t2373 / 48.0_f64 + t3077 * t9838 / 48.0_f64 - t3055 * t8611 / 48.0_f64 - t3921 * t2373 / 48.0_f64 - t3921 * t2379 / 96.0_f64 - t9849 * t2362 / 96.0_f64 + t8598 + 7.0_f64 / 144.0_f64 * t9852 + 7.0_f64 / 72.0_f64 * t9854 + t9858 * t833 / 96.0_f64;
    (t9856, t9858, t9861)
}
