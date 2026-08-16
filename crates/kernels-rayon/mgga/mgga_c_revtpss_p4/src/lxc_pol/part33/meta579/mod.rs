//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta579 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1989;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1990;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta579(t25299: f64, t92894: f64, t10073: f64, t1958: f64, t25390: f64, t886: f64, t1955: f64, t25308: f64, t2769: f64, t7049: f64, t786: f64, t867: f64, t2439: f64, t25334: f64, t887: f64, t7036: f64, t820: f64, t844: f64, t2482: f64, t814: f64, t10744: f64, t2664: f64, t7028: f64, t25240: f64, t2693: f64, t2710: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92895, t92905, t92917, t92921) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1989(t25299, t92894, t10073, t1958, t25390, t886, t1955, t25308, t2769, t7049, t786, t867);
        let (t92935, t92951, t92955, t92963, t92966) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1990(t2439, t25334, t887, t7036, t820, t844, t2482, t814, t10744, t2664, t7028, t25240, t2693, t2710);
    (t92895, t92905, t92917, t92921, t92935, t92951, t92955, t92963, t92966)
}
