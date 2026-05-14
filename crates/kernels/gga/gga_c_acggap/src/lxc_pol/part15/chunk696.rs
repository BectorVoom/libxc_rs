//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 696/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk696<F: Float>(t4210: F, t8306: F, t7942: F, t7965: F, t7963: F, t119: F, t2217: F, t2219: F, t310: F, t635: F, t848: F, t633: F) -> (F, F, F, F, F, F, F, F) {
    let t8310 = t8306 * t4210;
    let t8311 = t7942 * t8310;
    let t8313 = t8306 * t7965;
    let t8314 = t7963 * t8313;
    let t8316 = t119 * t2217;
    let t8319 = t310 * t2219;
    let t8330 = 0.65854491829355115987e0 * t848 * t635;
    let t8331 = t310 * t633;
    (t8310, t8311, t8313, t8314, t8316, t8319, t8330, t8331)
}
