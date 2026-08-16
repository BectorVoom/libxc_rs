//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1293/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1293(t1008: f64, t27772: f64, t2811: f64, t6533: f64, t100986: f64, t26685: f64, t1003: f64, t100447: f64, t100451: f64, t101066: f64, t101111: f64, t19396: f64, t27832: f64, t27904: f64, t44657: f64, t4939: f64, t7703: f64, t8038: f64, t93463: f64, t95640: f64, t95877: f64, t95892: f64) -> (f64, f64) {
    let t101231 = t27772 * t2811 * t6533 * t1008;
    let t101235 = t26685 * t100986;
    let t101237 = -t95877 + 0.30891203703703703704e-3_f64 * t7703 * t101066 + 0.18534722222222222223e-2_f64 * t7703 * t4939 * t93463 * t19396 - 0.46336805555555555556e-3_f64 * t95640 * t8038 - 0.92673611111111111112e-3_f64 * t27832 * t27904 + 0.41703125000000000001e-2_f64 * t7703 * t44657 * t101111 * t1003 - 0.1492375e-1_f64 * t100447 + 0.13901041666666666667e-2_f64 * t7703 * t101231 + 0.33163888888888888888e-2_f64 * t100451 - t95892 + 0.20612155671296296296e-4_f64 * t101235;
    (t101231, t101237)
}
