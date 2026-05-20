//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta839 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2968;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2969;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta839<F: Float>(t1889: F, t46595: F, t1353: F, t13767: F, t2661: F, t48432: F, t13768: F, t3889: F, t13977: F, t9962: F, t13847: F, t1399: F, t48919: F, t9816: F, t13850: F, t2482: F, t2668: F, t4000: F, t13841: F, t4010: F, t808: F, t13785: F, t48862: F, t13817: F, t13999: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t48947, t48951, t48955, t48971, t48975) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2968::<F>(t1889, t46595, t1353, t13767, t2661, t48432, t13768, t3889, t13977, t9962, t13847, t1399, t48919, t9816);
        let (t48982, t48984, t48999, t49001, t49003) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2969::<F>(t13850, t2482, t2668, t4000, t13841, t9962, t4010, t808, t13785, t48862, t13817, t13999);
    (t48947, t48951, t48955, t48971, t48975, t48982, t48984, t48999, t49001, t49003)
}
