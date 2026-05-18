//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 973/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk973<F: Float>(t2122: F, t2131: F, t2132: F, t847: F, t7990: F, t7994: F, t2130: F, t851: F, t7998: F, t7987: F, t7984: F, t3644: F, t609: F) -> (F, F, F, F, F, F) {
    let t32161 = t2131 * t2132 * t2122 * t847;
    let t32163 = t7990 * t7994;
    let t32165 = t851 * t2130;
    let t32167 = F::new(0.26020884564615598386e1) * t32165 * t7998;
    let t32168 = t7987 * t7998;
    let t32171 = t7990 * t7984;
    let t32176 = F::new(0.8673628188205199462e0) * t2131 * t2132 * t609 * t3644;
    (t32161, t32163, t32167, t32168, t32171, t32176)
}
