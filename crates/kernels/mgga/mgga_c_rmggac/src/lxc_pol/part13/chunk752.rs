//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 752/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk752<F: Float>(t2134: F, t27: F, t3118: F, t551: F, t2350: F, t4905: F, t26283: F, t2347: F, t26287: F, t798: F, t31057: F, t4048: F, t7494: F, t8526: F, t2060: F, t5249: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t38784 = t2134 * t27 * t3118 * t551;
    let t38792 = t2350 * t4905;
    let t38793 = t26283 * t38792;
    let t38795 = t2347 * t4905;
    let t38796 = t26287 * t38795;
    let t38798 = t2350 * t798;
    let t38799 = t31057 * t38798;
    let t38801 = t2350 * t4048;
    let t38802 = t26287 * t38801;
    let t38807 = t7494 * t8526;
    let t38812 = t2060 * t5249;
    (t38784, t38792, t38793, t38795, t38796, t38798, t38799, t38801, t38802, t38807, t38812)
}
