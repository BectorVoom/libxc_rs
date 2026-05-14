//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 785/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk785<F: Float>(t1882: F, t6156: F, t6085: F, t6094: F, t1424: F, t761: F) -> (F, F, F, F) {
    let t24731 = t1882 * t6156;
    let t24733 = t1882 * t6085;
    let t24735 = t1882 * t6094;
    let t24737 = t761 * t1424;
    (t24731, t24733, t24735, t24737)
}
