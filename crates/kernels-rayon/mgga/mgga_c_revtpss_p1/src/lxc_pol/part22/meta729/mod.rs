//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta729 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2786;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta729(t10115: f64, t225: f64, t880: f64, t10866: f64, t232: f64, t235: f64, t2723: f64, t2482: f64, t2719: f64, t596: f64, t10852: f64, t10832: f64, t10845: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40317, t40318, t40321, t40322, t40325, t40336, t40337, t40357) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2786(t10115, t225, t880, t10866, t232, t235, t2723, t2482, t2719, t596, t10852, t10832, t10845);
    (t40317, t40318, t40321, t40322, t40325, t40336, t40337, t40357)
}
