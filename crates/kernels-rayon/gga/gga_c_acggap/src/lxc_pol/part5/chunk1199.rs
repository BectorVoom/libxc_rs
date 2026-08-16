//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1199/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1199(t3382: f64, t5895: f64, t5899: f64, t1173: f64, t1180: f64, t1181: f64, t13261: f64, t13264: f64, t16755: f64, t16757: f64, t16759: f64, t1894: f64, t3396: f64, t407: f64, t4267: f64, t4680: f64, t4757: f64, t5270: f64, t5862: f64, t5894: f64, t6119: f64, t930: f64) -> f64 {
    let t21801 = t3382 * t5895;
    let t21815 = t3382 * t5899;
    let t21825 = 0.12862205435420921092e-2_f64 * t13261 + 0.17149607247227894789e-2_f64 * t13264 - 0.80031500487063509016e-2_f64 * t16755 + 0.20007875121765877254e-2_f64 * t16757 - 0.85748036236139473944e-3_f64 * t21801 + 0.16006300097412701803e-1_f64 * t16759 - 0.85748036236139473944e-3_f64 * t1180 * t4680 * t5894 - 0.85748036236139473944e-3_f64 * t1180 * t1181 * t6119 * t407 - 0.42874018118069736972e-3_f64 * t1180 * t1181 * t1894 * t930 + 0.85748036236139473944e-3_f64 * t21815 + 0.34299214494455789578e-2_f64 * t1173 * t1181 * t5862 * t5270 + 0.13719685797782315831e-1_f64 * t3396 * t1181 * t4267 * t4757;
    t21825
}
