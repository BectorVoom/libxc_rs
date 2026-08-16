//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta837 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2964;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2965;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta837(t1412: f64, t808: f64, t13927: f64, t48862: f64, t1389: f64, t14224: f64, t46835: f64, t13769: f64, t2453: f64, t547: f64, t9794: f64, t14230: f64, t2735: f64, t46801: f64, t40763: f64, t5609: f64, t9793: f64, t13830: f64, t9775: f64, t13826: f64, t3989: f64, t13937: f64, t9962: f64, t13991: f64, t13999: f64, t13786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48863, t48865, t48868, t48872, t48876) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2964(t1412, t808, t13927, t48862, t1389, t14224, t46835, t13769, t2453, t547, t9794, t14230, t2735, t46801);
        let (t48879, t48881, t48888, t48892, t48900, t48902) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2965(t40763, t5609, t9793, t13830, t9775, t13826, t3989, t13937, t9962, t13991, t13999, t13786);
    (t48863, t48865, t48868, t48872, t48876, t48879, t48881, t48888, t48892, t48900, t48902)
}
