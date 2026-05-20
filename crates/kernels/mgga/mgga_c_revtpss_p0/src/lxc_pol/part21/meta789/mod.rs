//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta789 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2838;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2839;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2840;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2841;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2842;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2843;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2844;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2845;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2846;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta789<F: Float>(t123: F, t127: F, t159: F, t1065: F, t11150: F, t13392: F, t606: F, t11144: F, t3181: F, t15194: F, t689: F, t49889: F, t905: F, t128: F, t904: F, t2435: F, t4584: F, t41281: F, t41283: F, t41285: F, t41287: F, t41289: F, t41292: F, t41307: F, t1593: F, t9292: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t51957 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2838::<F>(t123, t127, t159);
        let (t51958, t51959) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2839::<F>(t1065, t11150, t13392, t606);
        let t51961 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2840::<F>(t51957, t51958, t51959);
        let (t51963, t51965) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2841::<F>(t11144, t3181, t51957, t51959);
        let t51967 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2842::<F>(t15194, t689);
        let (t51969, t51971) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2843::<F>(t49889, t905, t128, t904);
        let t51973 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2844::<F>(t2435, t4584);
        let t51975 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2845::<F>(t51973, t41281, t41283, t41285, t41287, t41289, t41292, t41307, t51961, t51965, t51967, t51971);
        let t51978 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2846::<F>(t1593, t9292);
    (t51957, t51958, t51959, t51961, t51963, t51965, t51967, t51969, t51971, t51973, t51975, t51978)
}
