//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1291/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1291(t1131: f64, t1165: f64, t1198: f64, t14284: f64, t14286: f64, t14288: f64, t1439: f64, t1524: f64, t1795: f64, t1849: f64, t23944: f64, t23946: f64, t23951: f64, t23959: f64, t336: f64, t367: f64, t372: f64, t4261: f64, t4262: f64, t4267: f64, t4463: f64, t4930: f64, t5549: f64, t5572: f64, t8927: f64) -> f64 {
    let t23968 = -t4261 * t4262 * t5549 * t372 / 6.0_f64 - t4261 * t4262 * t1849 * t1131 / 12.0_f64 + t4261 * t8927 * t5572 * t372 / 4.0_f64 - t4261 * t4262 * t1439 * t1524 / 6.0_f64 + 0.16006300097412701803e-1_f64 * t23944 + 0.25724410870841842184e-2_f64 * t23946 + 0.85748036236139473944e-3_f64 * t23951 - 0.34299214494455789578e-1_f64 * t4463 * t1165 * t4267 * t4930 - 0.17149607247227894789e-1_f64 * t23959 - t367 * t336 * t1198 * t1795 / 96.0_f64 - 0.17149607247227894789e-2_f64 * t14284 + 0.85748036236139473944e-3_f64 * t14286 - 0.85748036236139473944e-3_f64 * t14288;
    t23968
}
