//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 749/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk749<F: Float>(t43: F, t12339: F, t12345: F, t2457: F, t3346: F, t47: F, t4757: F, t3351: F, t954: F, zeta_threshold: F) -> (F, F) {
    let t44 = t43 <= zeta_threshold;
    let t12349 = piecewise3::<f64>(t44, F::new(0.0), -F::new(8.0) / F::new(27.0) * t4757 * t12339 + F::new(4.0) / F::new(3.0) * t2457 * t3346 + F::new(4.0) / F::new(3.0) * t47 * t12345);
    let t12350 = t3351 * t954;
    (t12349, t12350)
}
