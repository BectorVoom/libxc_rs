//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta410 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2010;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2011;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2012;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta410(t14224: f64, t4100: f64, t2782: f64, t10014: f64, t5741: f64, t13790: f64, t1398: f64, t10022: f64, t10066: f64, t10070: f64, t10074: f64, t10080: f64, t10085: f64, t10098: f64, t10102: f64, t14066: f64, t14203: f64, t14209: f64, t14218: f64, t14221: f64, t213: f64, t546: f64, t1892: f64, t4086: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14225, t14227, t14229, t14230) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2010(t14224, t4100, t2782, t10014, t5741, t13790, t1398);
        let (t14231, t14233, t14237) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2011(t10022, t14230, t2782, t10066, t10070, t10074, t10080, t10085, t10098, t10102, t14066, t14203, t14209, t14218, t14221, t14227, t14229, t213, t546);
        let (t14238, t14239) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2012(t1892, t4086, t786);
    (t14225, t14227, t14229, t14230, t14231, t14233, t14237, t14238, t14239)
}
