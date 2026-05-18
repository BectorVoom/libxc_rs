//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 880/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk880<F: Float>(t1620: F, t5064: F, t617: F, t7853: F, t1630: F, t1791: F, t4929: F, t639: F, t5109: F, t642: F, t422: F, t5111: F, t626: F) -> (F, F, F) {
    let t16796 = F::new(256.0) / F::new(81.0) * t1620 * t7853 * t5064 * t617;
    let t16797 = t1630 * t1791;
    let t16799 = t639 * t16797 * t4929;
    let t16800 = F::new(64.0) / F::new(45.0) * t16799;
    let t16801 = t642 * t5109;
    let t16806 = F::new(32.0) / F::new(15.0) * t639 * t16801 * t5111 * t626 * t422;
    (t16796, t16800, t16806)
}
