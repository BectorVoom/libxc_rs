//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1385/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1385<F: Float>(t11882: F, t15983: F, t15986: F, t16627: F, t16629: F, t17995: F, t18002: F, t21819: F, t21822: F, t21825: F, t21834: F, t1571: F, t7444: F) -> (F, F) {
    let t22813 = -F::new(0.38691203703703703703e-3) * t21819 + F::new(0.34822083333333333332e-2) * t21822 + F::new(0.92858888888888888886e-2) * t21825 - F::new(0.25794135802469135802e-3) * t11882 - F::new(0.41270617283950617283e-2) * t21834 + F::new(0.20635308641975308642e-2) * t15983 - F::new(0.61905925925925925925e-2) * t15986 - t17995 + F::new(0.61905925925925925925e-2) * t16627 - F::new(0.41270617283950617283e-2) * t16629 - t18002;
    let t22833 = t7444 * t1571;
    (t22813, t22833)
}
