//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1097/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1097<F: Float>(t1626: F, t3011: F, t15125: F, t15191: F, t4644: F, t945: F, t1614: F, t2967: F, t2986: F, t4587: F, t914: F, t1596: F, t2923: F) -> (F, F, F, F, F, F, F, F) {
    let t15350 = t1626 * t3011;
    let t15363 = F::cast_from(0.2283111111111111111e-1_f64) * t15125;
    let t15364 = F::cast_from(0.11415555555555555555e-1_f64) * t15191;
    let t15400 = t4644 * t945;
    let t15406 = t1614 * t2967;
    let t15413 = t1626 * t2986;
    let t15416 = t4587 * t914;
    let t15421 = t1596 * t2923;
    (t15350, t15363, t15364, t15400, t15406, t15413, t15416, t15421)
}
