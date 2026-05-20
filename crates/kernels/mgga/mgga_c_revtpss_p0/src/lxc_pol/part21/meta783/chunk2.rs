//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2811/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2811<F: Float>(t231: F, t2782: F, t2783: F, t51625: F, t14946: F, t2710: F, t9285: F, t40938: F, t40942: F, t51617: F, t51621: F, t51623: F, t51628: F, t51632: F, t51635: F, t51637: F) -> F {
    let t51642 = t2782 * t2783 * t51625 * t231;
    let t51646 = t2710 * t14946 * t9285;
    let t51648 = -F::cast_from(0.29272321618148349057e-1_f64) * t51617 - F::cast_from(0.58544643236296698113e-1_f64) * t51621 - F::cast_from(0.29272321618148349057e-1_f64) * t51623 - F::cast_from(0.32927245914677557992e-1_f64) * t51628 + F::cast_from(0.11708928647259339623e0_f64) * t51632 + F::cast_from(0.46263278077393568556e-2_f64) * t51635 + F::cast_from(0.19514881078765566037e-2_f64) * t51637 - F::cast_from(0.19514881078765566037e-2_f64) * t40938 + F::cast_from(0.16463622957338778996e-1_f64) * t51642 + F::cast_from(0.9757440539382783019e-2_f64) * t40942 - F::cast_from(0.46263278077393568556e-2_f64) * t51646;
    t51648
}
