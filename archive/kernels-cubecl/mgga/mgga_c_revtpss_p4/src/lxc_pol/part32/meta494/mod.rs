//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta494 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1767;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1768;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1769;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta494<F: Float>(t1310: F, t7983: F, t7315: F, t8108: F, t13648: F, t2107: F, t28683: F, t508: F, t22496: F, t26405: F, t5542: F, t7536: F, t1453: F, t2014: F, t2322: F, t25082: F, t28652: F, t4248: F, t4254: F, t4293: F, t4297: F, t649: F, t651: F, t7359: F, t7378: F, t7969: F, t7984: F, t8065: F, t8075: F, t2089: F, t4292: F, t670: F, t1518: F, t7474: F, t1519: F, t26399: F, t28658: F, t4257: F, t7235: F, t7374: F, t7537: F, t7539: F, t7732: F, t7898: F, t7978: F, t7988: F, t8111: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t28704, t28707, t28709, t28711, t28718, t28727) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1767::<F>(t1310, t7983, t7315, t8108, t13648, t2107, t28683, t508, t22496, t26405, t5542, t7536);
        let t28729 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1768::<F>(t1310, t1453, t2014, t2322, t25082, t28652, t28704, t28707, t28709, t28711, t28718, t28727, t4248, t4254, t4293, t4297, t508, t649, t651, t7359, t7378, t7969, t7984, t8065, t8075);
        let (t28734, t28737, t28750, t28759) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1769::<F>(t2089, t4292, t670, t8065, t1518, t7474, t1519, t2322, t26399, t28658, t4254, t4257, t651, t7235, t7359, t7374, t7537, t7539, t7732, t7898, t7978, t7988, t8111);
    (t28704, t28707, t28709, t28711, t28718, t28727, t28729, t28734, t28737, t28750, t28759)
}
