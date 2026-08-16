//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 919/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk919<F: Float>(t177: F, t5305: F, t1634: F, t2661: F, t1037: F, t5384: F, t1769: F, t2663: F, t2575: F, t51: F, t2660: F, t568: F) -> (F, F, F, F, F, F) {
    let t6990 = t5305 * t177;
    let t6992 = t6990 * t2661 * t1634;
    let t6995 = t5384 * t1037;
    let t6998 = F::cast_from(0.40015750243531754508e-1_f64) * t1769 * t2663;
    let t6999 = t51 * t2575;
    let t7001 = t2660 * t6999 * t568;
    (t6990, t6992, t6995, t6998, t6999, t7001)
}
