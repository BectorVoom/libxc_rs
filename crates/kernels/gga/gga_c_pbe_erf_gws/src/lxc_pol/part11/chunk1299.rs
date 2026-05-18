//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1299/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1299<F: Float>(t10021: F, t10025: F, t13726: F, t16356: F, t16358: F, t16362: F, t16363: F, t16366: F, t16368: F, t16369: F, t16370: F, t16371: F, t16372: F, t7997: F, t8004: F, t8012: F, t8014: F) -> F {
    let t50805 = F::new(12.0) * t7997 - F::new(36.0) * t13726 - F::new(0.70178680769462448852e1) * t10021 - F::new(0.49291594608080000001e1) * t10025 - t16356 - t16358 + F::new(0.29298488058805055905e-2) * t8004 - t16362 - t16363 + t16366 - t16368 + t16369 + t16370 - t16371 - t16372 - F::new(96.0) * t8012 + F::new(144.0) * t8014;
    t50805
}
