//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1167/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1167(t1163: f64, t1181: f64, t4289: f64, t5725: f64, t4396: f64, t5743: f64, t1532: f64, t322: f64, t5799: f64, t1524: f64, t944: f64, t1165: f64, t1531: f64, t1552: f64, t16057: f64, t16072: f64, t16083: f64, t16110: f64, t16117: f64, t3396: f64, t406: f64, t4263: f64, t4298: f64, t5740: f64, t5741: f64, t6337: f64, t929: f64) -> f64 {
    let t21060 = t1163 * t1181 * t4289 * t5725;
    let t21066 = t4396 * t5743;
    let t21071 = t1163 * t1181 * t1532 * t5799 * t322;
    let t21077 = t944 * t1524;
    let t21093 = 0.17149607247227894789e-2_f64 * t21060 - 0.41159057393346947494e-1_f64 * t3396 * t1181 * t6337 * t4263 - 0.34299214494455789578e-2_f64 * t21066 + 0.17149607247227894789e-2_f64 * t21071 - 0.34299214494455789578e-2_f64 * t1531 * t1165 * t4298 * t5741 - 0.34299214494455789578e-2_f64 * t1531 * t1165 * t1552 * t21077 * t406 - 0.17149607247227894789e-2_f64 * t1531 * t1165 * t1552 * t5740 * t929 + 0.24009450146119052705e0_f64 * t16057 - 0.17149607247227894789e-2_f64 * t16072 + 0.16006300097412701803e-1_f64 * t16083 + 0.42874018118069736972e-3_f64 * t16110 + 0.68598428988911579156e-2_f64 * t16117;
    t21093
}
