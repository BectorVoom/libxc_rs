//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 923/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk923<F: Float>(t1786: F, t6524: F, t22943: F, t463: F, t1851: F, t100: F, t37429: F, t1326: F, t1587: F, t487: F, t6454: F, t26041: F) -> (F, F, F, F, F, F, F) {
    let t102848 = t1786 * t6524;
    let t102862 = t463 * t22943;
    let t102921 = t6524 * t1851;
    let t102948 = t37429 * t100;
    let t103073 = t1587 * t1326;
    let t103108 = t487 * t6454;
    let t103163 = t26041 * t487;
    (t102848, t102862, t102921, t102948, t103073, t103108, t103163)
}
