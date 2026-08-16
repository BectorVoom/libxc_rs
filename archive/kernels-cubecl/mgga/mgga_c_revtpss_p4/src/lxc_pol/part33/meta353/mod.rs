//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta353 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1370;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1371;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta353<F: Float>(t125: F, t5658: F, t2689: F, t5618: F, t1413: F, t5591: F, t547: F, t807: F, t5609: F, t808: F, t9845: F, t1885: F, t9909: F, t1399: F, t1872: F, t9818: F, t9816: F, t5706: F, t9962: F, t4000: F, t820: F, t844: F, t5677: F, t13847: F, t13848: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13944, t13949, t13951, t13954, t13956, t13959) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1370::<F>(t125, t5658, t2689, t5618, t1413, t5591, t547, t807, t5609, t808, t9845, t1885, t9909);
        let (t13985, t13987, t13988, t14001, t14005) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1371::<F>(t1399, t1872, t9818, t9816, t5706, t9962, t4000, t820, t844, t5677, t13847, t13848);
    (t13944, t13949, t13951, t13954, t13956, t13959, t13985, t13987, t13988, t14001, t14005)
}
