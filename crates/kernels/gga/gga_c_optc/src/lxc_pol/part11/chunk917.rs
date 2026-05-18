//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 917/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk917<F: Float>(t17219: F, t2813: F, t17134: F, t3836: F, t2722: F, t17169: F, t17148: F, t2674: F, t16917: F, t2601: F, t914: F, t2633: F) -> (F, F, F, F, F, F, F, F) {
    let t17220 = t2813 * t17219;
    let t17223 = t3836 * t17134;
    let t17226 = t2722 * t17219;
    let t17229 = t2722 * t17169;
    let t17232 = t17148 * t2674;
    let t17235 = t2601 * t16917;
    let t17236 = t914 * t17235;
    let t17239 = t2633 * t16917;
    (t17220, t17223, t17226, t17229, t17232, t17235, t17236, t17239)
}
