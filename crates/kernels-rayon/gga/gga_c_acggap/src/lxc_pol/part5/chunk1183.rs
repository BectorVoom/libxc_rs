//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1183/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1183(t14223: f64, t5749: f64, t1017: f64, t1165: f64, t1173: f64, t1180: f64, t13156: f64, t13161: f64, t16637: f64, t16639: f64, t16641: f64, t16644: f64, t16646: f64, t16648: f64, t16663: f64, t17544: f64, t1759: f64, t21128: f64, t4267: f64, t4680: f64, t5611: f64) -> f64 {
    let t21504 = t14223 * t5749;
    let t21513 = -0.85748036236139473944e-3_f64 * t13156 + 0.85748036236139473944e-3_f64 * t13161 + 0.64025200389650807212e-1_f64 * t16637 + 0.32012600194825403606e-1_f64 * t16639 + 0.32012600194825403606e-1_f64 * t16641 - 0.18140473443734395377e0_f64 * t16644 + 0.16006300097412701803e-1_f64 * t16646 + 0.16006300097412701803e-1_f64 * t16648 - 0.13719685797782315831e-1_f64 * t16663 + 0.10289764348336736873e-1_f64 * t1180 * t1165 * t17544 * t1759 * t1017 + 0.32012600194825403606e-1_f64 * t21504 - 0.34299214494455789578e-2_f64 * t1173 * t1165 * t4267 * t21128 + 0.34299214494455789578e-2_f64 * t1180 * t4680 * t5611;
    t21513
}
