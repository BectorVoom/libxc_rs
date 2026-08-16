//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2664/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2664(t13847: f64, t13848: f64, t3924: f64, t9816: f64, t13910: f64, t808: f64, t9736: f64, t14026: f64, t9744: f64, t125: f64, t13716: f64, t13975: f64, t1399: f64, t3934: f64, t3936: f64, t4004: f64, t4057: f64, t47259: f64, t47262: f64, t47277: f64, t47282: f64, t47284: f64, t47286: f64, t49012: f64, t49016: f64, t49024: f64, t49030: f64, t49049: f64, t5671: f64, t5673: f64, t5674: f64, t9891: f64) -> f64 {
    let t49053 = t9816 * t13847 * t13848 * t3924;
    let t49056 = t9736 * t808 * t13910;
    let t49057 = 0.30492001685571196935e-4_f64 * t49056;
    let t49058 = t9744 * t14026;
    let t49060 = -0.30492001685571196935e-4_f64 * t49012 + 0.22869001264178397701e-3_f64 * t49016 - 0.51448821741683684367e-2_f64 * t5671 * t3936 * t13975 * t4004 - 0.30492001685571196935e-3_f64 * t49024 - 0.21437009059034868486e-3_f64 * t3934 * t5673 * t5674 * t9891 + 455.0_f64 / 648.0_f64 * t49030 - 0.54214778996945588148e-4_f64 * t47259 + 0.97586602194502058666e-3_f64 * t47262 - 0.76230004213927992337e-3_f64 * t47277 - 0.38115002106963996168e-4_f64 * t47282 + 0.25724410870841842183e-2_f64 * t3934 * t3936 * t125 * t13716 * t1399 + 0.25724410870841842183e-2_f64 * t3934 * t3936 * t13975 * t4057 - 0.12004725073059526352e-1_f64 * t47284 + 0.30011812682648815881e-2_f64 * t47286 + 0.30492001685571196935e-3_f64 * t49049 - 0.38115002106963996168e-4_f64 * t49053 + t49057 - 7.0_f64 / 8.0_f64 * t49058;
    t49060
}
