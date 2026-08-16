//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1349/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1349<F: Float>(t14888: F, t15036: F, t19704: F, t20113: F, t29845: F, t52188: F, t52529: F, t53945: F, t53950: F, t53963: F, t53966: F, t53968: F, t55717: F, t55722: F, t55726: F, t55729: F, t55734: F, t6793: F, t8629: F, t8793: F) -> F {
    let t55738 = t8629 * t52188 / F::cast_from(48.0_f64) + t8793 * t52529 / F::cast_from(48.0_f64) + t53945 / F::cast_from(128.0_f64) + t19704 * t15036 / F::cast_from(48.0_f64) + t19704 * t14888 / F::cast_from(48.0_f64) + t20113 * t15036 / F::cast_from(48.0_f64) + t6793 * t55717 / F::cast_from(24.0_f64) + t6793 * t55722 / F::cast_from(24.0_f64) + t53950 / F::cast_from(12.0_f64) + t55726 + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t53963 - t53966 / F::cast_from(24.0_f64) - t29845 * t55729 / F::cast_from(32.0_f64) - t6793 * t55734 / F::cast_from(12.0_f64) + t53968 / F::cast_from(12.0_f64);
    t55738
}
