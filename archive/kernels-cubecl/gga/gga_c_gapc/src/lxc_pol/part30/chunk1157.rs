//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1157/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1157<F: Float>(t11465: F, t21084: F, t612: F, t11549: F, t8751: F, t11399: F, t5700: F, t1678: F, t11397: F, t632: F, t11533: F, t424: F) -> (F, F, F, F, F) {
    let t34328 = t21084 * t612 * t11465;
    let t34330 = t11549 * t8751;
    let t34333 = t11399 * t5700;
    let t34334 = t34333 * t1678;
    let t34335 = t632 * t11397 * t34334;
    let t34337 = t424 * t11533;
    (t34328, t34330, t34333, t34335, t34337)
}
