//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 701/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk701<F: Float>(t4978: F, t526: F, t4913: F, t541: F, t4929: F, t4934: F, t4937: F, t4939: F, t4943: F, t4945: F, t4947: F, t4950: F, t491: F, t472: F) -> (F, F, F, F, F) {
    let t4979 = t4978 * t526;
    let t4982 = t4913 * t541;
    let t4993 = -0.25319e1 * t4929 + 0.16879333333333333333e1 * t4934 - 0.19692555555555555555e1 * t4937 - 0.93011851851851851854e0 * t4939 + 0.13651666666666666667e0 * t4943 - 0.27303333333333333333e0 * t4945 - 0.3185388888888888889e0 * t4947 - 0.36514074074074074075e0 * t4950;
    let t4994 = t4993 * t491;
    let t4996 = 1.0 * t472 * t4994;
    (t4979, t4982, t4993, t4994, t4996)
}
