//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 341/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk341<F: Float>(t2961: F, t332: F, t113: F, t505: F, t910: F, t14: F, t1576: F, t17: F) -> (F, F, F, F, F) {
    let t2962 = t2961 * t332;
    let t2963 = t2962 * t113;
    let t2966 = t910 * t505;
    let t2998 = 1.0 / t14 / t1576;
    let t2999 = t2998 * t17;
    (t2962, t2963, t2966, t2998, t2999)
}
