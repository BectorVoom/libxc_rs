//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta622 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2380;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2381;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta622(t10733: f64, t9775: f64, t10716: f64, t10741: f64, t10665: f64, t243: f64, t231: f64, t2661: f64, t2662: f64, t10737: f64, t2652: f64, t212: f64, t2237: f64, t225: f64, t816: f64, t2665: f64, t10627: f64, t10697: f64, t236: f64, t807: f64, t10689: f64, t237: f64, t247: f64, t10709: f64, t10744: f64, t808: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40475, t40477, t40479, t40482, t40484, t40488) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2380(t10733, t9775, t10716, t10741, t10665, t243, t231, t2661, t2662, t10737, t2652, t212, t2237, t225, t816);
        let (t40489, t40503, t40507, t40509) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2381(t2665, t40488, t10627, t10697, t236, t807, t10689, t237, t247, t10709, t10744, t808);
    (t40475, t40477, t40479, t40482, t40484, t40488, t40489, t40503, t40507, t40509)
}
