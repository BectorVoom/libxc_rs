//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1289/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1289<F: Float>(t1614: F, t2967: F, t1626: F, t2986: F, t4587: F, t914: F, t936: F, t2919: F, t4590: F, t1596: F, t2923: F, t2927: F) -> (F, F, F, F, F) {
    let t15406 = t1614 * t2967;
    let t15413 = t1626 * t2986;
    let t15416 = t4587 * t914;
    let t15418 = F::new(2.0) * t15416 * t936;
    let t15420 = F::new(1.0) * t4590 * t2919;
    let t15421 = t1596 * t2923;
    let t15423 = F::cast_from(0.16081979498692535067e2_f64) * t15421 * t2927;
    (t15406, t15413, t15418, t15420, t15423)
}
