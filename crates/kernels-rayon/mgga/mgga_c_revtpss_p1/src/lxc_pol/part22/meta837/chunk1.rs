//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2965/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2965(t40763: f64, t5609: f64, t9793: f64, t13830: f64, t9775: f64, t13826: f64, t3989: f64, t13937: f64, t9962: f64, t13991: f64, t13999: f64, t13786: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48879 = t9793 * t40763 * t5609;
    let t48881 = t9775 * t13830;
    let t48888 = t3989 * t13826;
    let t48892 = t9962 * t13937;
    let t48900 = t13999 * t13991;
    let t48902 = t9962 * t13786;
    (t48879, t48881, t48888, t48892, t48900, t48902)
}
