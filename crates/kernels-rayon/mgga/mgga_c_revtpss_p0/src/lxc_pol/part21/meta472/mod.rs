//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2032;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2033;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta472(t14540: f64, t14572: f64, t14953: f64, t14976: f64, t868: f64, t4533: f64, t72: f64, t686: f64, t2465: f64, t1569: f64, t867: f64, t786: f64, t2467: f64, t122: f64, t4480: f64, t2466: f64, t10995: f64, t11044: f64, t4481: f64, t10498: f64, t10501: f64, t14474: f64, t14479: f64, t14484: f64, t14486: f64, t14489: f64, t865: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14978, t14979, t14982, t14983, t14985, t14986, t14987) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2032(t14540, t14572, t14953, t14976, t868, t4533, t72, t686, t2465, t1569, t867, t786);
        let (t14990, t14991, t14997) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2033(t14987, t2467, t122, t4480, t2466, t10995, t11044, t4481, t10498, t10501, t14474, t14479, t14484, t14486, t14489, t14979, t14985, t865);
    (t14978, t14979, t14982, t14983, t14986, t14987, t14990, t14991, t14997)
}
