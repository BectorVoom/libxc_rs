//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta839 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2968;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2969;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta839(t1889: f64, t46595: f64, t1353: f64, t13767: f64, t2661: f64, t48432: f64, t13768: f64, t3889: f64, t13977: f64, t9962: f64, t13847: f64, t1399: f64, t48919: f64, t9816: f64, t13850: f64, t2482: f64, t2668: f64, t4000: f64, t13841: f64, t4010: f64, t808: f64, t13785: f64, t48862: f64, t13817: f64, t13999: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48947, t48951, t48955, t48971, t48975) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2968(t1889, t46595, t1353, t13767, t2661, t48432, t13768, t3889, t13977, t9962, t13847, t1399, t48919, t9816);
        let (t48982, t48984, t48999, t49001, t49003) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2969(t13850, t2482, t2668, t4000, t13841, t9962, t4010, t808, t13785, t48862, t13817, t13999);
    (t48947, t48951, t48955, t48971, t48975, t48982, t48984, t48999, t49001, t49003)
}
