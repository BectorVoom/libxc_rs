//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta362 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1722;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1723;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1724;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta362(t1021: f64, t3201: f64, t362: f64, t40: f64, t361: f64, t351: f64, t1058: f64, t3231: f64, t1054: f64, t2434: f64, t371: f64, t373: f64, t367: f64, t1020: f64, t3230: f64, t11924: f64, t11927: f64, t11930: f64, t11933: f64, t11938: f64, t11941: f64, t11944: f64, t11947: f64, t11952: f64, t11954: f64, t3120: f64, t3208: f64, t375: f64, t3123: f64, t3168: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11956, t11960, t11961, t11962, t11965, t11967, t11970) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1722(t1021, t3201, t362, t40, t361, t351, t1058, t3231, t1054, t2434, t371, t373);
        let (t11972, t11973, t11976) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1723(t11970, t367, t1020, t3230, t11924, t11927, t11930, t11933, t11938, t11941, t11944, t11947, t11952, t11954, t11956, t11962, t11965, t11967, t3120, t3208, t375);
        let t11977 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1724(t3123, t3168);
    (t11956, t11960, t11961, t11962, t11965, t11967, t11970, t11972, t11973, t11976, t11977)
}
