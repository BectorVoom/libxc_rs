//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1015/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1015<F: Float>(t136188: F, t136189: F, t25990: F, t32067: F, t137245: F, t25894: F, t32069: F, t631: F, t92173: F, t1564: F, t25878: F, t3052: F, t32115: F) -> (F, F, F) {
    let t144781 = t32067 * t136188 * t136189 * t25990;
    let t144786 = t92173 * t631 * t137245 * t32069 * t25894;
    let t144790 = t25878 * t1564 * t32115 * t3052;
    (t144781, t144786, t144790)
}
