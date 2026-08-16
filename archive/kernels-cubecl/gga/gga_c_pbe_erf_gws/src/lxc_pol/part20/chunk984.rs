//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 984/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk984<F: Float>(t2593: F, t2612: F, t1627: F, t3519: F, t643: F, t9801: F, t642: F, t639: F, t3523: F, t1791: F, t3390: F, t617: F) -> (F, F, F, F, F) {
    let t11122 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t2612 * t2593;
    let t11124 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1627 * t3519;
    let t11125 = t643 * t9801;
    let t11126 = t642 * t11125;
    let t11128 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t639 * t11126;
    let t11130 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1627 * t3523;
    let t11131 = t1791 * t3390;
    let t11132 = t11131 * t617;
    (t11122, t11124, t11128, t11130, t11132)
}
