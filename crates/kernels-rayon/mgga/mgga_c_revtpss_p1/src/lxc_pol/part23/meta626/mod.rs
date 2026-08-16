//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta626 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2315;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2316;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta626(t5486: f64, t6573: f64, t1287: f64, t1811: f64, t6622: f64, t13149: f64, t24911: f64, t6587: f64, t1280: f64, t24713: f64, t13129: f64, t1774: f64, t21541: f64, t24616: f64, t1234: f64, t1285: f64, t12987: f64, t13127: f64, t13142: f64, t13148: f64, t17934: f64, t1818: f64, t1822: f64, t1825: f64, t20850: f64, t21439: f64, t24912: f64, t24915: f64, t24919: f64, t3670: f64, t460: f64, t5326: f64, t5436: f64, t6564: f64, t6720: f64, t6727: f64, t6731: f64, t6735: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24922, t24928, t24931, t24934, t24941, t24948, t24951) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2315(t5486, t6573, t1287, t1811, t6622, t13149, t24911, t6587, t1280, t24713, t13129, t1774, t21541);
        let (t24956, t24961) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2316(t1280, t24616, t1234, t1285, t12987, t13127, t13142, t13148, t17934, t1818, t1822, t1825, t20850, t21439, t24912, t24915, t24919, t24922, t24928, t24931, t24934, t24941, t24948, t24951, t3670, t460, t5326, t5436, t6564, t6720, t6727, t6731, t6735);
    (t24922, t24928, t24931, t24934, t24941, t24948, t24951, t24956, t24961)
}
