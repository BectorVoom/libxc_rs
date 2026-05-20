//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta385 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1287;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1288;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta385<F: Float>(t5486: F, t6573: F, t1287: F, t1811: F, t6622: F, t13149: F, t24911: F, t6587: F, t1280: F, t24713: F, t13129: F, t1774: F, t21541: F, t24616: F, t1234: F, t1285: F, t12987: F, t13127: F, t13142: F, t13148: F, t17934: F, t1818: F, t1822: F, t1825: F, t20850: F, t21439: F, t24912: F, t24915: F, t24919: F, t3670: F, t460: F, t5326: F, t5436: F, t6564: F, t6720: F, t6727: F, t6731: F, t6735: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t24922, t24928, t24931, t24934, t24941, t24948, t24951) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1287::<F>(t5486, t6573, t1287, t1811, t6622, t13149, t24911, t6587, t1280, t24713, t13129, t1774, t21541);
        let (t24956, t24961) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1288::<F>(t1280, t24616, t1234, t1285, t12987, t13127, t13142, t13148, t17934, t1818, t1822, t1825, t20850, t21439, t24912, t24915, t24919, t24922, t24928, t24931, t24934, t24941, t24948, t24951, t3670, t460, t5326, t5436, t6564, t6720, t6727, t6731, t6735);
    (t24922, t24928, t24931, t24934, t24941, t24948, t24951, t24956, t24961)
}
