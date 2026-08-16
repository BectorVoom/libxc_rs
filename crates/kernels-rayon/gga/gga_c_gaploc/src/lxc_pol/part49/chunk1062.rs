//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1062/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1062(t1445: f64, t25405: f64, t3209: f64, t5748: f64, t13034: f64, t15751: f64, t10948: f64, t9972: f64, t41451: f64, t41454: f64, t41457: f64, t41460: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44142 = 0.27606906686822939767e2_f64 * t5748 * t1445 * t25405 * t3209;
    let t44144 = 0.27606906686822939767e2_f64 * t15751 * t13034;
    let t44145 = t10948 * t9972;
    let t44148 = 0.89376224879626066674e-1_f64 * t41451;
    let t44149 = 0.59584149919750711116e-1_f64 * t41454;
    let t44150 = 0.15337170381568299871e1_f64 * t41457;
    let t44151 = 0.3575048995185042667e0_f64 * t41460;
    (t44142, t44144, t44145, t44148, t44149, t44150, t44151)
}
