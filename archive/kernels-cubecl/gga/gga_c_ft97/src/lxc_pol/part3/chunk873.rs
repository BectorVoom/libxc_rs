//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 873/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk873<F: Float>(t15625: F, t632: F, t72: F, t4872: F, t8618: F, t637: F, t643: F, t4861: F, t8675: F, t358: F, t4883: F, t363: F) -> (F, F, F, F) {
    let t17564 = t72 * t632 * t15625;
    let t17567 = t8618 * t4872;
    let t17569 = t637 * t17567 * t643;
    let t17573 = t8675 * t4861;
    let t17575 = t4883 * t358;
    let t17576 = t17575 * t363;
    (t17564, t17569, t17573, t17576)
}
