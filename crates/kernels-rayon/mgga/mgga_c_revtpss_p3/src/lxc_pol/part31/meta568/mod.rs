//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1980;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1981;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta568(t7059: f64, t9288: f64, t7064: f64, t25305: f64, t92868: f64, t136: f64, t2457: f64, t7082: f64, t25299: f64, t10073: f64, t1958: f64, t25390: f64, t886: f64, t1955: f64, t25308: f64, t2769: f64, t7049: f64, t786: f64, t867: f64, t2439: f64, t25334: f64, t887: f64, t7036: f64, t820: f64, t844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92871, t92873, t92875, t92894, t92895, t92905) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1980(t7059, t9288, t7064, t25305, t92868, t136, t2457, t7082, t25299, t10073, t1958, t25390, t886);
        let (t92917, t92921, t92935, t92951) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1981(t1955, t25308, t2769, t7049, t786, t867, t2439, t25334, t887, t7036, t820, t844);
    (t92871, t92873, t92875, t92894, t92895, t92905, t92917, t92921, t92935, t92951)
}
