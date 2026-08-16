//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 974/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk974<F: Float>(t7075: F, t8986: F, t2523: F, t3513: F, t967: F, t2521: F, t2515: F, t3517: F, t1409: F, t7150: F, t2479: F, t7148: F) -> (F, F, F, F, F, F, F) {
    let t8988 = F::cast_from(0.96491876992155210402e2_f64) * t7075 * t8986;
    let t8989 = t3513 * t2523;
    let t8990 = t8989 * t967;
    let t8992 = F::cast_from(0.32163958997385070134e2_f64) * t2521 * t8990;
    let t8993 = t3517 * t2515;
    let t8995 = F::cast_from(0.16081979498692535067e2_f64) * t2521 * t8993;
    let t8996 = t1409 * t7150;
    let t8997 = t8996 * t2479;
    let t8999 = F::cast_from(0.51726012919273400301e3_f64) * t7148 * t8997;
    (t8988, t8990, t8992, t8993, t8995, t8997, t8999)
}
