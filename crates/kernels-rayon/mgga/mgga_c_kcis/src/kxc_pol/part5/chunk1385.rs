//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1385/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1385(t11882: f64, t15983: f64, t15986: f64, t16627: f64, t16629: f64, t17995: f64, t18002: f64, t21819: f64, t21822: f64, t21825: f64, t21834: f64, t1571: f64, t7444: f64) -> (f64, f64) {
    let t22813 = -0.38691203703703703703e-3_f64 * t21819 + 0.34822083333333333332e-2_f64 * t21822 + 0.92858888888888888886e-2_f64 * t21825 - 0.25794135802469135802e-3_f64 * t11882 - 0.41270617283950617283e-2_f64 * t21834 + 0.20635308641975308642e-2_f64 * t15983 - 0.61905925925925925925e-2_f64 * t15986 - t17995 + 0.61905925925925925925e-2_f64 * t16627 - 0.41270617283950617283e-2_f64 * t16629 - t18002;
    let t22833 = t7444 * t1571;
    (t22813, t22833)
}
