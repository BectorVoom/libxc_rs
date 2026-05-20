//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2681/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2681<F: Float>(t3091: F, t43240: F, t6267: F, t16088: F, t380: F, t4746: F, t1065: F, t372: F, t6299: F, t3105: F, t6317: F, t15794: F, t15926: F) -> (F, F, F, F, F) {
    let t66763 = t3091 * t43240 * t6267;
    let t66766 = t4746 * t380 * t16088;
    let t66777 = t372 * t1065 * t6299;
    let t66784 = t6317 * t3105;
    let t66814 = t15926 * t15794;
    (t66763, t66766, t66777, t66784, t66814)
}
