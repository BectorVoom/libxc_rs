//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 516/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk516(t3773: f64, t4162: f64, t504: f64, t1455: f64, t1458: f64, t1520: f64, t1457: f64, t503: f64, t475: f64, t3502: f64, t382: f64, t487: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4163 = t3773 + t4162;
    let t4164 = t4163 * t504;
    let t4165 = t1455 * t1458;
    let t4167 = 2.0_f64 * t4165 * t1520;
    let t4169 = 1.0_f64 / t1457 / t503;
    let t4170 = t475 * t4169;
    let t4171 = t1520 * t1520;
    let t4173 = 2.0_f64 * t4170 * t4171;
    let t4174 = t382 * t3502;
    let t4175 = t487 * t4174;
    (t4163, t4164, t4165, t4167, t4169, t4170, t4171, t4173, t4174, t4175)
}
