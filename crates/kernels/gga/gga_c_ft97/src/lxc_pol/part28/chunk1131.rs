//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1131/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1131<F: Float>(t1969: F, t27142: F, t3052: F, t32946: F, t2185: F, t23657: F, t27147: F, t148270: F, t148275: F, t148278: F, t148282: F, t148286: F, t148290: F, t148295: F, t148299: F, t148304: F, t148309: F, t148311: F, t148315: F, t148319: F, t148323: F) -> (F, F, F) {
    let t148327 = t27142 * t1969 * t32946 * t3052;
    let t148331 = t23657 * t2185 * t32946 * t27147;
    let t148333 = -F::new(2.0) * t148270 - F::new(5.0) / F::new(4.0) * t148275 - t148278 / F::new(9.0) - t148282 / F::new(3.0) - t148286 / F::new(3.0) - F::new(2.0) / F::new(9.0) * t148290 - F::new(20.0) / F::new(3.0) * t148295 + F::new(8.0) / F::new(3.0) * t148299 + F::new(4.0) / F::new(9.0) * t148304 + t148309 / F::new(18.0) + t148311 / F::new(27.0) + t148315 / F::new(2.0) - t148319 / F::new(36.0) + F::new(8.0) / F::new(3.0) * t148323 + t148327 / F::new(9.0) - t148331 / F::new(6.0);
    (t148327, t148331, t148333)
}
