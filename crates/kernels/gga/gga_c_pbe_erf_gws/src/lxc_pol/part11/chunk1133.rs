//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1133/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1133<F: Float>(t10419: F, t3564: F, t12711: F, t2741: F, t10743: F, t186: F, t220: F, t47638: F, t616: F, t3451: F, t40402: F, t10969: F) -> (F, F, F, F, F, F) {
    let t48067 = F::new(8.0) / F::new(5.0) * t10419 * t3564;
    let t48069 = F::new(16.0) / F::new(15.0) * t2741 * t12711;
    let t48071 = F::new(16.0) / F::new(15.0) * t10743 * t12711;
    let t48076 = -F::new(4.0) / F::new(15.0) * t616 * t186 * t220 * t47638;
    let t48078 = F::new(16.0) / F::new(5.0) * t40402 * t3451;
    let t48080 = F::new(8.0) / F::new(5.0) * t10969 * t3451;
    (t48067, t48069, t48071, t48076, t48078, t48080)
}
