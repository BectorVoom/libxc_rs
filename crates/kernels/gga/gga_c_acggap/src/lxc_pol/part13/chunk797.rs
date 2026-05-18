//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 797/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk797<F: Float>(t1524: F, t604: F, t142: F, t2060: F, t1314: F, t7815: F, t7450: F, t1318: F, t2030: F, t1545: F, t7561: F, t1549: F, t7822: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8630 = t604 * t1524;
    let t8631 = t142 * t8630;
    let t8632 = t2060 * t8631;
    let t8634 = t7815 * t1314;
    let t8635 = t7450 * t8634;
    let t8637 = t7815 * t1318;
    let t8638 = t2030 * t8637;
    let t8640 = t7561 * t1545;
    let t8642 = t7822 * t1549;
    (t8630, t8631, t8632, t8634, t8635, t8637, t8638, t8640, t8642)
}
