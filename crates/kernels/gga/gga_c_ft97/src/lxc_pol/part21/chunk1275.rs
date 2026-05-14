//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1275/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1275<F: Float>(t1882: F, t30196: F, t1369: F, t30191: F, t376: F, t17099: F, t5899: F, t5900: F, t9432: F, t2185: F, t23657: F, t27147: F, t27165: F, t30211: F, t379: F, t95292: F, t95293: F) -> (F, F, F, F, F, F, F) {
    let t119796 = t1882 * t30196;
    let t119797 = t119796 / 9.0;
    let t119799 = t1369 * t376 * t30191;
    let t119800 = t119799 / 6.0;
    let t119803 = t5899 * t9432 * t5900 * t17099;
    let t119807 = t23657 * t2185 * t27165 * t27147;
    let t119810 = t95292 * t95293 * t30211 * t379;
    (t119796, t119797, t119799, t119800, t119803, t119807, t119810)
}
