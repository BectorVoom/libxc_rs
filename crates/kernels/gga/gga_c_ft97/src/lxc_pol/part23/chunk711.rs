//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 711/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk711<F: Float>(t18497: F, t3885: F, t2606: F, t18459: F, t2599: F, t4917: F, t766: F) -> (F, F, F, F, F) {
    let t18498 = t3885 * t18497;
    let t18499 = t2606 * t18498;
    let t18502 = t3885 * t18459;
    let t18503 = t2599 * t18502;
    let t18506 = t4917 * t766;
    (t18498, t18499, t18502, t18503, t18506)
}
