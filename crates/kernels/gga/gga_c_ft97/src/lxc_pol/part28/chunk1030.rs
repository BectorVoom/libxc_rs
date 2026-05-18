//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1030/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1030<F: Float>(t1317: F, t144958: F, t1800: F, t28: F, t136138: F, t144846: F, t32067: F, t144666: F, t432: F, t89: F, t3103: F, t32355: F) -> (F, F, F, F) {
    let t144961 = t1317 * t28 * t1800 * t144958;
    let t144966 = t32067 * t136138 * t144846;
    let t144970 = t89 * t28 * t144666 * t432;
    let t144974 = t89 * t28 * t32355 * t3103;
    (t144961, t144966, t144970, t144974)
}
