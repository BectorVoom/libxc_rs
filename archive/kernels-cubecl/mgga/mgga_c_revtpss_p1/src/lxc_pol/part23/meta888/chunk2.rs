//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2814/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2814<F: Float>(t2723: F, t2782: F, t4503: F, t76131: F, t1558: F, t6041: F, t231: F, t2783: F, t4500: F, t62967: F, t10661: F, t14972: F, t23172: F, t4366: F, t4504: F, t51299: F, t6017: F, t62606: F, t62609: F, t76117: F, t76125: F, t76127: F, t820: F) -> (F, F) {
    let t76134 = t2782 * t4503 * t76131 * t2723;
    let t76136 = t6041 * t1558;
    let t76139 = t2782 * t2783 * t76136 * t231;
    let t76144 = t62967 * t4500;
    let t76147 = -F::cast_from(0.54878743191129263322e-2_f64) * t76117 - F::cast_from(0.19756347548806534796e1_f64) * t820 * t14972 * t6017 - F::cast_from(0.32927245914677557992e-1_f64) * t62606 + F::cast_from(0.58544643236296698112e-1_f64) * t76125 + F::cast_from(0.13170898365871023197e1_f64) * t4504 * t76127 * t4366 - t51299 - F::cast_from(0.32927245914677557992e-1_f64) * t76134 + F::cast_from(0.16463622957338778997e-1_f64) * t76139 + F::cast_from(0.39512695097613069591e1_f64) * t820 * t10661 * t23172 - F::cast_from(0.29272321618148349057e-1_f64) * t76144 - F::cast_from(0.58544643236296698114e-1_f64) * t62609;
    (t76136, t76147)
}
