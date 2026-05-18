//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1118/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1118<F: Float>(t20049: F, t20053: F, t20058: F, t20063: F, t20076: F, t2359: F, t2379: F, t2388: F, t2392: F, t4405: F, t4410: F, t4459: F, t6112: F, t6135: F, t6145: F, t6789: F, t6802: F, t827: F) -> F {
    let t20080 = -t6802 * t2379 / F::new(24.0) + t2388 * t6145 / F::new(8.0) - F::new(35.0) / F::new(18.0) * t20049 - t827 * t20053 / F::new(4.0) - t827 * t20058 / F::new(12.0) - t2359 * t20063 / F::new(16.0) - t6112 * t2379 / F::new(24.0) - t4405 * t4459 / F::new(12.0) - t4410 * t4459 / F::new(12.0) - t2392 * t6135 / F::new(4.0) - t2392 * t6789 / F::new(8.0) + F::new(7.0) / F::new(24.0) * t20076 + t2392 * t6145 / F::new(8.0);
    t20080
}
