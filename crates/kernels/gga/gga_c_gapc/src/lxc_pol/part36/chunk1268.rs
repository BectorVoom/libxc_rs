//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1268/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1268<F: Float>(t10110: F, t11691: F, t2493: F, t3243: F, t640: F, t10336: F, t11695: F, t3209: F, t35846: F, t923: F, t22949: F, t22954: F, t268: F, t35424: F, t6148: F, t7875: F) -> (F, F, F, F, F) {
    let t35940 = t10110 * t11691;
    let t35943 = t3243 * t640 * t2493;
    let t35945 = t10336 * t11695;
    let t35948 = t3209 * t35846 * t923;
    let t35954 = t35424 * t268 * t22949 * t6148 * t7875 * t22954;
    (t35940, t35943, t35945, t35948, t35954)
}
