//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1090/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1090(t9931: f64, t9934: f64, t9936: f64, t12740: f64, t12741: f64, t12742: f64, t12743: f64, t12745: f64, t12746: f64, t12748: f64, t12749: f64, t12750: f64, t12751: f64) -> (f64, f64, f64, f64) {
    let t12752 = 8.0_f64 / 45.0_f64 * t9931;
    let t12753 = 16.0_f64 / 135.0_f64 * t9934;
    let t12754 = 8.0_f64 / 45.0_f64 * t9936;
    let t12755 = t12740 - t12741 + t12742 - t12743 - t12745 - t12746 - t12748 + t12749 - t12750 - t12751 - t12752 - t12753 + t12754;
    (t12752, t12753, t12754, t12755)
}
