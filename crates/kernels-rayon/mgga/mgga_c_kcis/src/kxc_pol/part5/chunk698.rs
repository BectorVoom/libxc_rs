//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 698/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk698(t1195: f64, t1812: f64, t1187: f64, t3438: f64, t4823: f64, t3437: f64, t143: f64, t4768: f64, t365: f64, t932: f64, t346: f64, t1110: f64, t1115: f64, t1143: f64, t1757: f64, t1761: f64, t1780: f64, t3381: f64, t348: f64, t4602: f64, t4607: f64, t4626: f64, t4638: f64, t4643: f64, t4671: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5096 = t1195 * t1812;
    let t5097 = t1187 * t5096;
    let t5099 = t3438 * t4823;
    let t5100 = t3437 * t5099;
    let t5102 = t4768 * t143;
    let t5111 = t365 * t932;
    let t5122 = t365 * t346;
    let t5127 = 0.619125e-2_f64 * t5102 * t348 + 0.9286875e-2_f64 * t1780 * t1110 - 0.619125e-2_f64 * t1780 * t1115 + 0.9286875e-2_f64 * t1143 * t1757 + 0.46434375e-2_f64 * t5111 * t4602 - 0.9286875e-2_f64 * t3381 * t4607 + 0.9286875e-2_f64 * t365 * t4626 - 0.619125e-2_f64 * t1143 * t1761 - 0.9286875e-2_f64 * t3381 * t4638 + 0.123825e-1_f64 * t5122 * t4643 - 0.619125e-2_f64 * t365 * t4671;
    (t5096, t5097, t5099, t5100, t5102, t5111, t5122, t5127)
}
