//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2662/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2662(t13785: f64, t48862: f64, t48999: f64, t13817: f64, t13999: f64, t13981: f64, t9962: f64, t13951: f64, t2713: f64, t3964: f64, t1353: f64, t14019: f64, t1872: f64, t3889: f64, t3944: f64, t47223: f64, t47227: f64, t47229: f64, t47231: f64, t47235: f64, t47239: f64, t47245: f64, t48971: f64, t48975: f64, t48982: f64, t48984: f64, t5689: f64, t800: f64, t9628: f64) -> f64 {
    let t49001 = t48862 * t48999 * t13785;
    let t49003 = t13999 * t13817;
    let t49005 = t9962 * t13981;
    let t49008 = t3964 * t2713 * t13951;
    let t49010 = -0.60023625365297631762e-2_f64 * t47223 - 0.12705000702321332056e-4_f64 * t47227 - 0.17006693853500995666e-1_f64 * t47229 - 0.24009450146119052704e-1_f64 * t48971 - 0.76230004213927992337e-4_f64 * t48975 - 0.12004725073059526352e-1_f64 * t47231 - 0.30492001685571196935e-3_f64 * t47235 + 0.76230004213927992336e-4_f64 * t47239 - 0.8131200449485652516e-3_f64 * t48982 - 0.12004725073059526352e-1_f64 * t48984 + 0.60023625365297631762e-1_f64 * t47245 + 3.0_f64 / 16.0_f64 * t3944 * t800 * t14019 * t1353 + 3.0_f64 / 16.0_f64 * t3944 * t800 * t5689 * t3889 + t3944 * t800 * t1872 * t9628 / 16.0_f64 + 0.85748036236139473944e-3_f64 * t49001 - 0.18007087609589289528e-1_f64 * t49003 - 0.12004725073059526352e-1_f64 * t49005 - 0.54214778996945588151e-4_f64 * t49008;
    t49010
}
