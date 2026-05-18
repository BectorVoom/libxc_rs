//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1089/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1089<F: Float>(t10589: F, t10615: F, t810: F, t788: F, t4143: F, t6574: F, t3443: F, t8769: F, t2311: F, t4193: F, t3444: F, t4180: F, t6666: F) -> (F, F, F, F, F, F, F, F) {
    let t10616 = t10589 + t10615;
    let t10617 = t10616 * t810;
    let t10619 = F::new(1.0) * t788 * t10617;
    let t10621 = F::new(0.16081979498692535067e2) * t6574 * t4143;
    let t10622 = t3443 * t8769;
    let t10625 = t2311 * t4193;
    let t10626 = t10625 * t3444;
    let t10629 = t6666 * t4180;
    (t10616, t10617, t10619, t10621, t10622, t10625, t10626, t10629)
}
