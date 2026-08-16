//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2447/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2447(t135: f64, t21458: f64, t973: f64, t20234: f64, t42841: f64, t2986: f64, t4514: f64, t61189: f64, t10186: f64, t10235: f64, t13798: f64, t17863: f64, t21433: f64, t21459: f64, t21476: f64, t2960: f64, t42811: f64, t42817: f64, t4510: f64, t48217: f64, t61074: f64, t61172: f64, t61210: f64, t68462: f64, t68466: f64, t68470: f64, t68481: f64, t68521: f64) -> f64 {
    let t69540 = t973 * t135 * t21458;
    let t69548 = t42841 * t20234;
    let t69570 = t2986 * t61189 * t4514;
    let t69574 = 0.13333333333333333332e-1_f64 * t2986 * t4510 * t68481 + 0.22222222222222222222e-2_f64 * t2960 * t21459 - 0.27777777777777777777e-3_f64 * t69540 - 0.66666666666666666663e-2_f64 * t2986 * t4510 * t68462 - 0.1037037037037037037e-1_f64 * t2986 * t13798 * t68521 + 0.22222222222222222221e-2_f64 * t2986 * t10235 * t69548 + 0.11111111111111111111e-2_f64 * t2986 * t4510 * t68466 + 0.11111111111111111111e-2_f64 * t2986 * t4510 * t68470 - 0.11111111111111111111e-2_f64 * t2986 * t48217 * t17863 - 0.83333333333333333331e-3_f64 * t2986 * t61210 * t4514 + 0.22222222222222222222e-2_f64 * t61074 + 0.44444444444444444443e-2_f64 * t10186 * t21476 + 0.22222222222222222222e-2_f64 * t10186 * t21433 - 0.27777777777777777777e-3_f64 * t69570 - 0.82304526748971193413e-3_f64 * t42811 - t42817 - 0.83333333333333333331e-3_f64 * t61172;
    t69574
}
