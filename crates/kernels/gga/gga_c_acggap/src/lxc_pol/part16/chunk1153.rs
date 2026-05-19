//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1153/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1153<F: Float>(t2001: F, t5811: F, t31160: F, t31168: F, t35380: F, t35385: F, t35388: F, t35391: F, t35393: F, t35395: F, t39907: F, t39910: F, t39914: F, t39919: F, t39923: F, t39925: F, t39928: F, t39930: F, t39932: F) -> F {
    let t39934 = t2001 * t5811;
    let t39936 = F::new(0.22921875e-1) * t39907 + F::new(0.1528125e-1) * t39910 - F::cast_from(0.17149607247227894789e-2_f64) * t31160 - F::cast_from(0.42874018118069736972e-3_f64) * t39914 - F::cast_from(0.7145669686344956162e-3_f64) * t31168 + F::cast_from(0.32155513588552302729e-2_f64) * t39919 - F::cast_from(0.32155513588552302729e-2_f64) * t39923 - t35380 - F::new(11.0) / F::new(576.0) * t39925 + t35385 + t35388 + t35391 - t35393 + F::new(0.22921875e-1) * t39928 + F::cast_from(0.68598428988911579156e-2_f64) * t39930 - t35395 - F::cast_from(0.34299214494455789578e-2_f64) * t39932 - F::cast_from(0.34299214494455789578e-2_f64) * t39934;
    t39936
}
