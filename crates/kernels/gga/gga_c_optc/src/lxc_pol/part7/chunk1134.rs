//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1134/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1134<F: Float>(t24521: F, t2672: F, t8101: F, t889: F, t2620: F, t2731: F, t287: F, t320: F, t321: F, t3695: F, t2737: F, t2743: F, t8229: F, t921: F, t7895: F, t947: F) -> (F, F, F, F, F, F, F) {
    let t25776 = t24521 * t2672;
    let t25781 = t8101 * t889;
    let t25783 = t2731 * t2620;
    let t25788 = 0.85858385084333410912e-1 * t320 * t321 * t3695 * t287;
    let t25789 = t2737 * t2743;
    let t25791 = t921 * t8229;
    let t25793 = t947 * t7895;
    (t25776, t25781, t25783, t25788, t25789, t25791, t25793)
}
