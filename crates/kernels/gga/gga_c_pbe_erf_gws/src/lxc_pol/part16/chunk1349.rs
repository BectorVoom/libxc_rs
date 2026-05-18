//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1349/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1349<F: Float>(t14888: F, t15036: F, t19704: F, t20113: F, t29845: F, t52188: F, t52529: F, t53945: F, t53950: F, t53963: F, t53966: F, t53968: F, t55717: F, t55722: F, t55726: F, t55729: F, t55734: F, t6793: F, t8629: F, t8793: F) -> F {
    let t55738 = t8629 * t52188 / F::new(48.0) + t8793 * t52529 / F::new(48.0) + t53945 / F::new(128.0) + t19704 * t15036 / F::new(48.0) + t19704 * t14888 / F::new(48.0) + t20113 * t15036 / F::new(48.0) + t6793 * t55717 / F::new(24.0) + t6793 * t55722 / F::new(24.0) + t53950 / F::new(12.0) + t55726 + F::new(5.0) / F::new(192.0) * t53963 - t53966 / F::new(24.0) - t29845 * t55729 / F::new(32.0) - t6793 * t55734 / F::new(12.0) + t53968 / F::new(12.0);
    t55738
}
