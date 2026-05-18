//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 659/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk659<F: Float>(t1882: F, t5075: F, t5153: F, t5070: F, t1131: F, t2567: F, t5064: F, t258: F, t4934: F, t5053: F, t5147: F, t761: F) -> (F, F, F, F, F, F, F, F) {
    let t18544 = t1882 * t5075;
    let t18593 = t1882 * t5153;
    let t18633 = t1882 * t5070;
    let t18675 = t2567 * t1131;
    let t18680 = t2567 * t5064;
    let t18685 = t258 * t4934;
    let t18729 = t258 * t5053;
    let t18740 = t761 * t5147;
    (t18544, t18593, t18633, t18675, t18680, t18685, t18729, t18740)
}
