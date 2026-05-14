//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 882/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk882<F: Float>(t1503: F, t7329: F, t1992: F, t5616: F, t7585: F, t7586: F, t10146: F, t167: F, t576: F, t137: F, t3300: F, t30407: F, t31097: F, t495: F, t7325: F, t30543: F, t8610: F) -> (F, F, F, F, F, F) {
    let t34659 = t7329 * t1503;
    let t34675 = t7585 * t7586 * t1992 * t5616;
    let t34691 = t576 * t167 * t10146;
    let t34692 = t3300 * t137;
    let t34698 = t30407 * t31097 * t7325 * t495;
    let t34702 = t30543 * t8610;
    (t34659, t34675, t34691, t34692, t34698, t34702)
}
