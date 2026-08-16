//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 1018/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk1018(t41460: f64, t41463: f64, t44110: f64, t44111: f64, t44112: f64, t44114: f64, t44118: f64, t44120: f64, t44124: f64, t44128: f64, t44131: f64, t44134: f64, t44138: f64, t44142: f64, t44144: f64, t44145: f64, t44147: f64, t44148: f64, t44149: f64, t44150: f64) -> f64 {
    let t44151 = 0.3575048995185042667e0_f64 * t41460;
    let t44152 = 0.17875244975925213335e0_f64 * t41463;
    let t44153 = t44110 - t44111 + t44112 - 0.89376224879626066674e-1_f64 * t44114 - t44118 + 0.51123901271894332901e0_f64 * t44120 + 0.85206502119823888169e-1_f64 * t44124 - 0.85206502119823888169e-1_f64 * t44128 - t44131 + t44134 + t44138 + t44142 + t44144 - 0.21450293971110256002e1_f64 * t44145 + t44147 - t44148 + t44149 + t44150 + t44151 - t44152;
    t44153
}
