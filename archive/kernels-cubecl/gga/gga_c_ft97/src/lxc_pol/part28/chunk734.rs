//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 734/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk734<F: Float>(t1564: F, t32115: F, t379: F, t5674: F, t432: F, t7211: F, t1800: F, t1317: F, t28: F, t22696: F, t7177: F, t22701: F, t6: F) -> (F, F, F, F, F, F, F) {
    let t32117 = t1564 * t32115 * t379;
    let t32118 = t5674 * t32117;
    let t32120 = t7211 * t432;
    let t32121 = t1800 * t32120;
    let t32123 = t1317 * t28 * t32121;
    let t32125 = t22696 * t7177;
    let t32128 = t22701 * t6;
    (t32117, t32118, t32120, t32121, t32123, t32125, t32128)
}
