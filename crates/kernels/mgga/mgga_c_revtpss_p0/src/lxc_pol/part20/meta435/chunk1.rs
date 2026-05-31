//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1640/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1640<F: Float>(t1012: F, t1222: F, t13095: F, t17261: F, t3699: F, t39443: F, t39449: F, t43847: F, t43852: F, t44898: F, t44902: F, t44906: F, t44912: F, t44917: F, t44919: F, t44925: F, t44928: F, t44931: F, t44938: F, t5308: F, t5312: F) -> F {
    let t44942 = -F::cast_from(0.16937883700965822014e-2_f64) * t44898 + F::cast_from(0.19055119163586549765e-3_f64) * t44902 + F::cast_from(0.3811023832717309953e-3_f64) * t44906 + t1222 * t5312 * t43852 / F::cast_from(6.0_f64) - t44912 / F::cast_from(36.0_f64) - t1222 * t5308 * t43847 / F::cast_from(36.0_f64) + F::cast_from(0.38110238327173099531e-3_f64) * t44917 - t1222 * t1012 * t44919 * t39443 / F::cast_from(12.0_f64) + t44925 / F::cast_from(216.0_f64) - t44928 / F::cast_from(216.0_f64) - F::cast_from(5.0_f64) / F::cast_from(972.0_f64) * t44931 + t1222 * t1012 * t3699 * t39449 / F::cast_from(72.0_f64) - F::cast_from(0.34299214494455789578e-2_f64) * t44938 + F::cast_from(0.51448821741683684368e-2_f64) * t17261 * t13095;
    t44942
}
