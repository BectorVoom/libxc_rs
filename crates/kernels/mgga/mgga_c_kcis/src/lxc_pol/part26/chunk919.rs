//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 919/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk919<F: Float>(t11882: F, t15983: F, t15986: F, t16627: F, t16629: F, t17995: F, t18002: F, t21819: F, t21822: F, t21825: F, t21834: F, t1571: F, t7444: F, t2080: F, t6097: F, t7463: F) -> (F, F, F, F) {
    let t22813 = -0.38691203703703703703e-3 * t21819 + 0.34822083333333333332e-2 * t21822 + 0.92858888888888888886e-2 * t21825 - 0.25794135802469135802e-3 * t11882 - 0.41270617283950617283e-2 * t21834 + 0.20635308641975308642e-2 * t15983 - 0.61905925925925925925e-2 * t15986 - t17995 + 0.61905925925925925925e-2 * t16627 - 0.41270617283950617283e-2 * t16629 - t18002;
    let t22833 = t7444 * t1571;
    let t22836 = t2080 * t6097;
    let t22839 = t7463 * t1571;
    (t22813, t22833, t22836, t22839)
}
