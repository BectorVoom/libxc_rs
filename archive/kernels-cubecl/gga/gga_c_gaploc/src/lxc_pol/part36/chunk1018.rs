//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 1018/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk1018<F: Float>(t41460: F, t41463: F, t44110: F, t44111: F, t44112: F, t44114: F, t44118: F, t44120: F, t44124: F, t44128: F, t44131: F, t44134: F, t44138: F, t44142: F, t44144: F, t44145: F, t44147: F, t44148: F, t44149: F, t44150: F) -> F {
    let t44151 = F::cast_from(0.3575048995185042667e0_f64) * t41460;
    let t44152 = F::cast_from(0.17875244975925213335e0_f64) * t41463;
    let t44153 = t44110 - t44111 + t44112 - F::cast_from(0.89376224879626066674e-1_f64) * t44114 - t44118 + F::cast_from(0.51123901271894332901e0_f64) * t44120 + F::cast_from(0.85206502119823888169e-1_f64) * t44124 - F::cast_from(0.85206502119823888169e-1_f64) * t44128 - t44131 + t44134 + t44138 + t44142 + t44144 - F::cast_from(0.21450293971110256002e1_f64) * t44145 + t44147 - t44148 + t44149 + t44150 + t44151 - t44152;
    t44153
}
