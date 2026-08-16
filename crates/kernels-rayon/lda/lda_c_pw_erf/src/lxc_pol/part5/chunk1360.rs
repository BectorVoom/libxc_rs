//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1360/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1360(t10976: f64, t10980: f64, t10983: f64, t10987: f64, t10988: f64, t10991: f64, t10995: f64, t11574: f64, t125: f64, t14488: f64, t1729: f64, t19421: f64, t19847: f64, t19850: f64, t19860: f64, t19864: f64, t19866: f64, t2205: f64, t23124: f64, t23152: f64, t23166: f64, t23176: f64, t23354: f64, t2644: f64, t4429: f64, t454: f64, t5783: f64, t5925: f64, t7881: f64) -> f64 {
    let t23358 = -5.4655730795145296e-05_f64 * t10976 - t10980 + 0.0001639671923854359_f64 * t10983 + t10987 - 0.15965645347006147_f64 * t10988 - t10991 - t10995 - 0.0008717022455366076_f64 * t19847 - 0.0017434044910732151_f64 * t19850 + 18.0_f64 * t23124 * t2205 - 18.0_f64 * t14488 * t19421 + 0.5945049527603057_f64 * t19860 + 0.004067943812504169_f64 * t19864 + 18.0_f64 * t1729 * t2644 * t454 * t5925 - 9.0_f64 * t11574 * t7881 - 9.0_f64 * t5783 * t19866 * t4429 + (t23152 + t23166 + t23176 + t23354) * t125;
    t23358
}
