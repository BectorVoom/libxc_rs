//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1301/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1301(t21101: f64, t21148: f64, t21467: f64, t21497: f64, t1386: f64, t17292: f64, t5637: f64, t4160: f64, t1307: f64, t7313: f64, t4170: f64, t17298: f64, t5668: f64) -> (f64, f64, f64, f64) {
    let t21499 = t21101 + t21148 + t21467 + t21497;
    let t21500 = t21499 * t1386;
    let t21507 = t17292 * t5637;
    let t21508 = t4160 * t21507;
    let t21510 = t7313 * t1307;
    let t21511 = t4170 * t21510;
    let t21512 = t4160 * t21511;
    let t21514 = t17298 * t5668;
    (t21500, t21508, t21512, t21514)
}
