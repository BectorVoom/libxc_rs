//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 746/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk746<F: Float>(t11175: F, t17: F, t9: F, t3141: F, t8282: F, t959: F, t2: F, t3103: F, t1587: F, t432: F, t1588: F, t3149: F, t7750: F) -> (F, F, F, F, F) {
    let t11717 = t9 * t11175 * t17;
    let t11718 = t11717 * t3141;
    let t11720 = t8282 * t959;
    let t11722 = t2 * t3103;
    let t11724 = t1587 * t11722 * t432;
    let t11728 = t7750 * t3149 * t1588;
    (t11717, t11718, t11720, t11724, t11728)
}
