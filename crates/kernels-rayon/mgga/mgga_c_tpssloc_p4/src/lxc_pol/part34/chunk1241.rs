//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1241/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1241(t102227: f64, t102275: f64, t106813: f64, t106826: f64, t106829: f64, t106842: f64, t106849: f64, t106853: f64, t106855: f64, t2032: f64, t23963: f64, t26911: f64, t27961: f64, t27976: f64, t27982: f64, t7026: f64, t7432: f64, t7782: f64, t91954: f64) -> f64 {
    let t108708 = 30.0_f64 * t91954 * t27961 + 30.0_f64 * t23963 * t106826 - 5.0_f64 * t7026 * t106813 - 5.0_f64 * t7026 * t106842 - 5.0_f64 * t26911 * t27976 - 5.0_f64 / 3.0_f64 * t7026 * t106849 - 2.0_f64 * t102227 * t106853 + 10.0_f64 * t102275 * t7432 - 2.0_f64 * t106829 * t2032 - 2.0_f64 / 3.0_f64 * t106855 * t2032 - 2.0_f64 * t27982 * t7782;
    t108708
}
