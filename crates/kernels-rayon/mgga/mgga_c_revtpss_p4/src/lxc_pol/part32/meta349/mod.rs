//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta349 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1283;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1284;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta349(t2470: f64, t5740: f64, t4101: f64, t1432: f64, t5763: f64, t1385: f64, t5710: f64, t1904: f64, t3899: f64, t689: f64, t3920: f64, t5603: f64, t2435: f64, t5718: f64, t1893: f64, t2453: f64, t3908: f64, t3895: f64, t2439: f64, t213: f64, t1532: f64, t2609: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14243, t14252, t14255, t14276, t14280) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1283(t2470, t5740, t4101, t1432, t5763, t1385, t5710, t1904, t3899, t689, t3920, t5603);
        let (t14290, t14294, t14297, t14299, t14312) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1284(t2435, t5718, t1893, t2453, t3908, t1904, t3895, t2439, t213, t5710, t1532, t2609);
    (t14243, t14252, t14255, t14276, t14280, t14290, t14294, t14297, t14299, t14312)
}
