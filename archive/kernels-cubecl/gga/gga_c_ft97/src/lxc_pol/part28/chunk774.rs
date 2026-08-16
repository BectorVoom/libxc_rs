//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 774/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk774<F: Float>(t1339: F, t452: F, t5617: F, t487: F, t7281: F, t379: F, t1909: F, t32412: F, t83: F, t110: F, t1871: F, t32120: F) -> (F, F, F, F, F, F) {
    let t32512 = t452 * t1339 * t5617;
    let t32515 = t487 * t7281;
    let t32516 = t32515 * t379;
    let t32517 = t1909 * t32516;
    let t32520 = t83 * t32412;
    let t32524 = t1871 * t110 * t32120;
    (t32512, t32515, t32516, t32517, t32520, t32524)
}
