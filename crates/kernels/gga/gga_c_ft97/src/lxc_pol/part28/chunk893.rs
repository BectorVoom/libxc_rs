//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 893/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk893<F: Float>(t22952: F, t22953: F, t34405: F, t379: F, t136151: F, t136159: F, t144796: F, t32067: F, t32069: F, t3266: F, t36450: F, t637: F, t136188: F, t136189: F, t25996: F, t25985: F, t32350: F) -> (F, F, F, F, F) {
    let t144829 = t22952 * t22953 * t34405 * t379;
    let t144832 = t136159 * t136151 * t144796;
    let t144836 = t32067 * t637 * t36450 * t32069 * t3266;
    let t144840 = t32067 * t136188 * t136189 * t25996;
    let t144844 = t22952 * t22953 * t32350 * t25985;
    (t144829, t144832, t144836, t144840, t144844)
}
