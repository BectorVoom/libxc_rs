//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta328 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1625;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1626;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta328(t2689: f64, t5618: f64, t1413: f64, t5591: f64, t547: f64, t807: f64, t5609: f64, t808: f64, t9845: f64, t1885: f64, t9909: f64, t1399: f64, t1872: f64, t9818: f64, t9816: f64, t5706: f64, t9962: f64, t4000: f64, t820: f64, t844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13949, t13951, t13952, t13954, t13955, t13956, t13959, t13985) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1625(t2689, t5618, t1413, t5591, t547, t807, t5609, t808, t9845, t1885, t9909, t1399, t1872, t9818);
        let (t13987, t13988, t13999) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1626(t13985, t9816, t5706, t9962, t4000, t820, t844);
    (t13949, t13951, t13952, t13954, t13955, t13956, t13959, t13985, t13987, t13988, t13999)
}
