//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 928/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk928(t68: f64, t9819: f64, t1287: f64, t1292: f64, t2488: f64, t1290: f64, t1342: f64, t1451: f64, t2513: f64, t2531: f64, t311: f64, t410: f64, t4945: f64, t4950: f64, t4981: f64, t4983: f64, t4997: f64, t5000: f64, t5014: f64, t5632: f64, t6121: f64, t6138: f64, t9764: f64, t9770: f64, t9777: f64, t9798: f64, t9816: f64) -> (f64, f64, f64) {
    let t9820 = t9819 * t68;
    let t9821 = t9820 * t1287;
    let t9823 = t2488 * t1292;
    let t9825 = 1.8805371096875316_f64 * t9764 * t311 - 19.489173774580152_f64 * t6138 * t2513 - 19.489173774580152_f64 * t1290 * t9770 - 1.8805371096875316_f64 * t6121 * t2513 - 1.8805371096875316_f64 * t1342 * t9770 + 7.35994946043302_f64 * t9777 - t4945 - 1.6457779058161184_f64 * t4950 + t4981 - 3.600163427964126_f64 * t4983 - t4997 - t5000 - t5014 - 2.9824072957409817_f64 * t2531 * t5632 - 2.9824072957409817_f64 * t9798 * t1451 - 1.8805371096875316_f64 * t9816 * t410 + 22.07984838129906_f64 * t9821 + 22.07984838129906_f64 * t9823;
    (t9821, t9823, t9825)
}
