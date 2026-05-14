//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1030/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1030<F: Float>(t3444: F, t5384: F, t496: F, t8775: F, t1542: F, t3426: F, t1508: F, t8770: F, t114: F, t557: F, t8748: F, t1499: F, t545: F, t83: F, t1532: F, t3380: F, t49: F) -> (F, F, F, F, F, F, F, F) {
    let t24489 = t5384 * t3444;
    let t24527 = t496 * t8775;
    let t24534 = t1542 * t3426;
    let t24536 = t8770 * t1508;
    let t24539 = t8748 * t114 * t557;
    let t24542 = t8770 * t1499;
    let t24600 = t83 * t8748 * t545;
    let t24604 = t3380 * t49 * t1532;
    (t24489, t24527, t24534, t24536, t24539, t24542, t24600, t24604)
}
