//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 680/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk680<F: Float>(t2801: F, t959: F, t2315: F, t195: F, t291: F, t286: F, t941: F) -> (F, F, F) {
    let t7177 = t959 * t2801;
    let t7178 = t7177 * t2315;
    let t7182 = t195 * t291;
    let t7191 = t941 * t286;
    (t7178, t7182, t7191)
}
