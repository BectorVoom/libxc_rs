//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 992/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk992(t110: f64, t1789: f64, t1793: f64, t209: f64, t508: f64, t6432: f64, t6435: f64, t514: f64, t535: f64, t622: f64, t1756: f64, t1759: f64) -> (f64, f64, f64, f64) {
    let t21891 = 0.2291123905095794067e1_f64 * t209 * t110 * t1789 * t1793;
    let t21895 = 0.68733717152873822009e1_f64 * t209 * t508 * t6432 * t6435;
    let t21899 = 0.22161481481481481481e0_f64 * t209 * t622 * t514 * t535;
    let t21903 = 0.28493333333333333334e0_f64 * t209 * t110 * t1756 * t1759;
    (t21891, t21895, t21899, t21903)
}
