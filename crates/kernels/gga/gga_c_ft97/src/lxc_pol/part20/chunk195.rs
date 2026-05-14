//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 195/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk195<F: Float>(t218: F, t227: F, t52: F, t11: F, t209: F, t41: F) -> (F, F, F) {
    let t1407 = t227 * t218;
    let t1408 = t52 * t1407;
    let t1410 = 0.45058854638888888889e-1 * t41 * t11 * t209 + 0.11113838714704711852e-2 * t1408;
    (t1407, t1408, t1410)
}
