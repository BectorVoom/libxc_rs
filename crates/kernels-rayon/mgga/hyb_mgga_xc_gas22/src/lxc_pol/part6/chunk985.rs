//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 985/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk985(t9135: f64, t950: f64, t957: f64, t2490: f64, t3485: f64, t2496: f64, t3490: f64, t952: f64, t3496: f64, t6969: f64, t6972: f64, t9113: f64, t9116: f64, t9119: f64, t9123: f64, t9127: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9136 = t950 * t9135;
    let t9138 = t957 * t9135;
    let t9140 = t3485 * t2490;
    let t9142 = t2496 * t3490;
    let t9143 = t9142 * t952;
    let t9145 = t3496 * t2490;
    let t9147 = -t9113 - t9116 + 0.24647e0_f64 * t9119 + 0.49294e0_f64 * t9123 + 0.24647e0_f64 * t9127 + 0.79724444444444444446e0_f64 * t6969 - 0.29896666666666666667e0_f64 * t6972 + 0.1898925e1_f64 * t9136 + 0.3071625e0_f64 * t9138 - 0.9494625e0_f64 * t9140 + 0.3071625e0_f64 * t9143 + 0.15358125e0_f64 * t9145;
    (t9136, t9138, t9140, t9143, t9145, t9147)
}
