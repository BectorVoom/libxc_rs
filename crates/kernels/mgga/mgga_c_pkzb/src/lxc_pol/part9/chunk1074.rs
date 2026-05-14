//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1074/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1074<F: Float>(t1020: F, t1029: F, t1031: F, t133: F, t158: F, t162: F, t17000: F, t1742: F, t20317: F, t20318: F, t20320: F, t20321: F, t20327: F, t20344: F, t20361: F, t20380: F, t2631: F, t2632: F, t2633: F, t2636: F, t5181: F, t5217: F, t5304: F, t5348: F, t5364: F, t594: F, t597: F, t7055: F, t7065: F, t7070: F, t7071: F, t7081: F) -> (F,) {
    let t20397 = -36.0 * t1742 * t133 * t2633 - 360.0 * t2631 * t5304 * t1020 * t5181 + 180.0 * t2631 * t7070 * t17000 + 180.0 * t7065 * t7071 + 9.0 * t594 * t7081 - (t20317 + t20318 + t20320 + t20321 + t20327 + t20344 + t20361 + t20380) * t158 * t162 - 12.0 * t2631 * t2632 * t5217 + 9.0 * t7055 * t597 + 9.0 * t1742 * t2636 + 3.0 * t1029 * t5364 + 3.0 * t5348 * t1031;
    (t20397,)
}
