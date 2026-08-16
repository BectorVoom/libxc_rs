//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1338/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1338(t10417: f64, t13750: f64, t22114: f64, t22115: f64, t22116: f64, t22117: f64, t22121: f64, t22125: f64, t22129: f64, t22133: f64, t22137: f64, t22141: f64, t22143: f64) -> f64 {
    let t23290 = t10417 + t22114 - t22115 - t22116 + t22117 + t22121 + t22125 - t22129 + t22133 + t22137 - t22141 - t13750 + t22143;
    t23290
}
