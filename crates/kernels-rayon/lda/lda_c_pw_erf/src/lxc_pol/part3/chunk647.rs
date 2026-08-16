//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 647/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk647(t1321: f64, t3854: f64, t1318: f64, t3796: f64, t3801: f64, t3805: f64, t3810: f64, t3814: f64, t3816: f64, t3821: f64, t3823: f64, t3827: f64, t3831: f64, t3836: f64, t3840: f64, t3843: f64, t3845: f64, t3849: f64, t3853: f64) -> (f64, f64, f64, f64) {
    let t3855 = t3854 * t1321;
    let t3856 = t1318 * t3855;
    let t3857 = 32.0_f64 / 45.0_f64 * t3856;
    let t3858 = -t3796 - t3801 - t3805 + t3810 + t3814 + t3816 + t3821 - t3823 - t3827 - t3831 - t3836 - t3840 - t3843 - t3845 - t3849 - t3853 + t3857;
    (t3855, t3856, t3857, t3858)
}
