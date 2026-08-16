//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1294/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1294(t10868: f64, t10872: f64, t10970: f64, t14516: f64, t14896: f64, t14899: f64, t14904: f64, t14906: f64, t14911: f64, t159: f64, t18809: f64, t19866: f64, t20179: f64, t20646: f64, t23100: f64, t23115: f64, t279: f64, t281: f64, t285: f64, t4430: f64, t5740: f64, t5783: f64, t6015: f64, t6016: f64, t6089: f64) -> f64 {
    let t23118 = t14516 - 9.0_f64 * t18809 * t4430 - 6.0_f64 * t5783 * t19866 * t6015 + 9.0_f64 * t6089 * t5740 - 6.0_f64 * t18809 * t6016 - 0.0008717022455366076_f64 * t10868 - t10872 - 0.01197423401025461_f64 * t281 * t20179 * t159 * t285 - 0.01197423401025461_f64 * t20646 - t14896 - 0.03592270203076383_f64 * t14899 - t14904 - 0.0001639671923854359_f64 * t14906 - 1.370765728342244e-05_f64 * t14911 + (t23100 + t23115) * t279 - t10970;
    t23118
}
