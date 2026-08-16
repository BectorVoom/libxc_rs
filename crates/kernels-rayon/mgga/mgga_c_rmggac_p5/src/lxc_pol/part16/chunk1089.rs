//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1089/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1089(t2211: f64, t30453: f64, t43157: f64, t43158: f64, t43169: f64, t45811: f64, t45813: f64, t45818: f64, t45822: f64, t45825: f64, t45827: f64, t45830: f64, t45832: f64, t45836: f64, t45844: f64, t45846: f64, t4985: f64, t5055: f64, t6473: f64, t739: f64, t9399: f64, t9402: f64, t9405: f64) -> f64 {
    let t48662 = 0.35922725105591425692e0_f64 * t5055 * t9399 - 0.47896966807455234256e0_f64 * t6473 * t9402 - 0.23948483403727617128e0_f64 * t4985 * t9405 - 0.5107751987195740728e-4_f64 * t45811 + 0.5107751987195740728e-4_f64 * t45813 + 0.1702583995731913576e-4_f64 * t45818 + 0.15323255961587222184e-3_f64 * t45822 + 0.1702583995731913576e-4_f64 * t45825 + 0.212822999466489197e-4_f64 * t45827 - t43157 + t43158 + 0.13637330827122670865e0_f64 * t45830 + 0.13637330827122670865e-1_f64 * t45832 - 0.10215503974391481456e-3_f64 * t45836 + t43169 - 0.1702583995731913576e-4_f64 * t45844 - 0.85129199786595678799e-5_f64 * t45846 + 0.11974241701863808564e0_f64 * t739 * t2211 * t30453;
    t48662
}
