//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta655 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2443;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2444;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta655(t11853: f64, t828: f64, t3229: f64, t360: f64, t3089: f64, t1087: f64, t11672: f64, t11711: f64, t1024: f64, t12003: f64, t11744: f64, t3188: f64, t3181: f64, t675: f64, t1063: f64, t247: f64, t2853: f64, t11151: f64, t11725: f64, t283: f64, t2852: f64, t11951: f64, t3211: f64, t1025: f64, t3218: f64, t371: f64, t676: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42410, t42415, t42416, t42417, t42421, t42425, t42439) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2443(t11853, t828, t3229, t360, t3089, t1087, t11672, t11711, t1024, t12003, t11744, t3188);
        let (t42450, t42454, t42471, t42477, t42481) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2444(t3181, t675, t1063, t247, t2853, t11151, t11725, t283, t2852, t11951, t3211, t1025, t3218, t371, t676);
    (t42410, t42415, t42416, t42417, t42421, t42425, t42439, t42450, t42454, t42471, t42477, t42481)
}
