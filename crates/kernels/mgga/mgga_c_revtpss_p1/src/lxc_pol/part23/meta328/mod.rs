//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta328 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1625;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1626;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta328<F: Float>(t2689: F, t5618: F, t1413: F, t5591: F, t547: F, t807: F, t5609: F, t808: F, t9845: F, t1885: F, t9909: F, t1399: F, t1872: F, t9818: F, t9816: F, t5706: F, t9962: F, t4000: F, t820: F, t844: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13949, t13951, t13952, t13954, t13955, t13956, t13959, t13985) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1625::<F>(t2689, t5618, t1413, t5591, t547, t807, t5609, t808, t9845, t1885, t9909, t1399, t1872, t9818);
        let (t13987, t13988, t13999) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1626::<F>(t13985, t9816, t5706, t9962, t4000, t820, t844);
    (t13949, t13951, t13952, t13954, t13955, t13956, t13959, t13985, t13987, t13988, t13999)
}
