//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1073/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1073<F: Float>(t1049: F, t5663: F, t1765: F, t3143: F, t1055: F, t20092: F, t345: F, t1769: F, t19510: F, t346: F, t13696: F, t13699: F, t13701: F, t13706: F, t13714: F, t13727: F, t13729: F, t13737: F, t16230: F) -> (F, F, F, F, F, F) {
    let t21707 = t1049 * t5663;
    let t21709 = t3143 * t1765;
    let t21712 = t345 * t1055 * t20092;
    let t21714 = t3143 * t1769;
    let t21717 = t345 * t346 * t19510;
    let t21720 = -0.21733333333333333334e1 * t13696 + 0.1956e1 * t13699 + 0.2445e0 * t13701 + 0.2445e0 * t13706 - 0.12225e0 * t13714 + t13727 - 0.489e0 * t13729 + t13737 + 0.978e0 * t21707 + 0.2282e1 * t21709 + 0.1467e1 * t21712 - 0.1141e1 * t21714 - 0.36675e0 * t21717 + 0.978e0 * t16230;
    (t21707, t21709, t21712, t21714, t21717, t21720)
}
