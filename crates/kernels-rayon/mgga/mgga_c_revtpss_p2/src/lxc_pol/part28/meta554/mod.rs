//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta554 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2005;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2006;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta554(t25305: f64, t92868: f64, t1032: f64, t2760: f64, t867: f64, t7063: f64, t7060: f64, t136: f64, t2457: f64, t7082: f64, t25299: f64, t212: f64, t25286: f64, t689: f64, t780: f64, t10073: f64, t1958: f64, t25390: f64, t886: f64, t1955: f64, t25308: f64, t2769: f64, t7049: f64, t786: f64, t2467: f64, t2772: f64, t7014: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92875, t92888, t92889, t92891, t92894, t92895, t92901) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2005(t25305, t92868, t1032, t2760, t867, t7063, t7060, t136, t2457, t7082, t25299, t212, t25286, t689, t780);
        let (t92905, t92917, t92921, t92922, t92925) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2006(t10073, t1958, t25390, t886, t1955, t25308, t2769, t7049, t786, t867, t2467, t2772, t689, t7014);
    (t92875, t92888, t92889, t92891, t92894, t92895, t92901, t92905, t92917, t92921, t92922, t92925)
}
