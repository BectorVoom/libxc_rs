//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 272/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk272(t231: f64, t517: f64, t570: f64, t606: f64, t613: f64, t615: f64, t788: f64, t797: f64, t801: f64, t810: f64, t815: f64, t824: f64, t828: f64, t837: f64, t838: f64) -> f64 {
    let t841 = t788 + t797 + t517 + t801 - t810 + t815 + t824 + t570 + t828 - t837 + 4.0_f64 / 3.0_f64 * t838 * t231 + t606 + t613 + t615;
    t841
}
