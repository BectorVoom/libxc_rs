//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1179/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1179(t1083: f64, t2840: f64, t4567: f64, t1788: f64, t752: f64, t1154: f64, t1155: f64, t1791: f64, t318: f64, t86: f64, t1110: f64, t1115: f64, t1143: f64, t14252: f64, t14274: f64, t1757: f64, t1761: f64, t1780: f64, t2429: f64, t3289: f64, t3299: f64, t3304: f64, t3308: f64, t3372: f64, t365: f64, t4626: f64, t4671: f64, t5102: f64, t5133: f64) -> f64 {
    let t14962 = t2840 * t1083;
    let t14963 = t14962 * t4567;
    let t14966 = t752 * t1788;
    let t14992 = t1154 * t1155;
    let t14996 = t86 * t318 * t1791;
    let t14998 = -0.88437037037037037036e-1_f64 * t5133 * t14963 + 0.5895802469135802469e-2_f64 * t14966 - 0.619125e-2_f64 * t1780 * t3308 + 0.9286875e-2_f64 * t3372 * t1757 + 0.1857375e-1_f64 * t1143 * t4626 + 0.1857375e-1_f64 * t5102 * t1110 - 0.123825e-1_f64 * t5102 * t1115 + 0.9286875e-2_f64 * t365 * t14252 - 0.619125e-2_f64 * t3372 * t1761 - 0.123825e-1_f64 * t1143 * t4671 + 0.9286875e-2_f64 * t1780 * t3299 + 0.123825e-1_f64 * t1780 * t3304 - 0.619125e-2_f64 * t365 * t14274 + 0.46434375e-2_f64 * t1780 * t3289 - 0.53062222222222222222e-1_f64 * t2429 * t14992 + 0.88437037037037037037e-2_f64 * t14996;
    t14998
}
