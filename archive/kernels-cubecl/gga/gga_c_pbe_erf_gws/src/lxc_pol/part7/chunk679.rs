//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 679/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk679<F: Float>(t5451: F, t714: F, t1791: F, t1793: F, t617: F, t1621: F, t1620: F, t1627: F, t1631: F, t1893: F, t155: F, t641: F) -> (F, F, F, F, F, F, F, F) {
    let t5452 = t5451 * t714;
    let t5454 = t1791 * t1793;
    let t5455 = t5454 * t617;
    let t5456 = t1621 * t5455;
    let t5458 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t1620 * t5456;
    let t5459 = t1627 * t1631;
    let t5460 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t5459;
    let t5462 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1627 * t1893;
    let t5463 = t155 * t641;
    (t5452, t5454, t5455, t5456, t5458, t5460, t5462, t5463)
}
