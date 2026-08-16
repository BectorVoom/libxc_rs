//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 750/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk750<F: Float>(t1478: F, t95: F, t194: F, t417: F, t1305: F, t201: F, t1297: F, t212: F, tau1: F) -> (F, F, F, F, F) {
    let t3961 = t95 * t1478;
    let t3962 = t417 * t194;
    let t3963 = F::cast_from(1.0_f64) / t3962;
    let t3965 = F::cast_from(1.0_f64) / t1305 / t201;
    let t3969 = t1297 * t212;
    let t3972 = tau1 * tau1;
    (t3961, t3963, t3965, t3969, t3972)
}
