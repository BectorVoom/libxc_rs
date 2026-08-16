//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta126 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk668;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk669;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk670;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta126(t378: f64, t4746: f64, t1647: f64, t1678: f64, t994: f64, t1668: f64, t73: f64, t1058: f64, t1660: f64, t1065: f64, t2857: f64, t2852: f64, t3181: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t4747 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk668(t378, t4746);
        let (t4752, t4778, t4781, t4792, t4801) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk669(t1647, t378, t1678, t994, t1668, t73, t1058, t1660, t1065, t2857);
        let t4806 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk670(t2852, t3181);
    (t4747, t4752, t4778, t4781, t4792, t4801, t4806)
}
