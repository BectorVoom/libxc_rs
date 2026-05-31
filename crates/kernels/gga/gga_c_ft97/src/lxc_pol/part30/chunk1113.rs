//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1113/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1113<F: Float>(t143177: F, t143180: F, t152899: F, t152902: F, t152905: F, t152907: F, t152913: F, t152917: F, t152920: F, t152924: F, t152927: F, t152931: F, t152934: F, t152937: F, t152940: F, t152943: F) -> F {
    let t152945 = -F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t152899 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t152902 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t152905 - t152907 / F::cast_from(54.0_f64) + t143177 / F::cast_from(18.0_f64) + t143180 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) * t152913 + t152917 / F::cast_from(18.0_f64) + t152920 / F::cast_from(18.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t152924 - t152927 / F::cast_from(9.0_f64) + t152931 / F::cast_from(9.0_f64) - t152934 / F::cast_from(36.0_f64) + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t152937 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t152940 - t152943 / F::cast_from(3.0_f64);
    t152945
}
