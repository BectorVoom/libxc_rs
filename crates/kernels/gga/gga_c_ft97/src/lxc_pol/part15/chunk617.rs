//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 617/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk617<F: Float>(t1882: F, t5176: F, t5149: F, t5066: F, t5172: F, t8392: F, t5167: F, t1160: F, t2486: F, t5087: F, t5083: F, t5079: F, t5075: F, t5153: F, t5070: F, t1131: F, t2567: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t18427 = t1882 * t5176;
    let t18431 = t1882 * t5149;
    let t18452 = t1882 * t5066;
    let t18455 = t8392 * t5172;
    let t18457 = t8392 * t5167;
    let t18467 = t2486 * t1160;
    let t18538 = t1882 * t5087;
    let t18540 = t1882 * t5083;
    let t18542 = t1882 * t5079;
    let t18544 = t1882 * t5075;
    let t18593 = t1882 * t5153;
    let t18633 = t1882 * t5070;
    let t18675 = t2567 * t1131;
    (t18427, t18431, t18452, t18455, t18457, t18467, t18538, t18540, t18542, t18544, t18593, t18633, t18675)
}
