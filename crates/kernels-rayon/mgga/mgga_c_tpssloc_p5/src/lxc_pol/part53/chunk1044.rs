//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1044/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1044(t671: f64, t8710: f64, t32255: f64, t33103: f64, t116905: f64, t116910: f64, t116917: f64, t116920: f64, t116929: f64, t116936: f64, t116945: f64, t116954: f64, t119880: f64, t119902: f64, t119917: f64, t119924: f64, t119928: f64, t119932: f64, t119933: f64, t119948: f64, t32245: f64, t32249: f64, t33111: f64, t8706: f64) -> (f64, f64) {
    let t124293 = t8710 * t671;
    let t124324 = t33103 * t32255;
    let t124330 = -40.0_f64 / 27.0_f64 * t116920 + t116917 - 20.0_f64 / 27.0_f64 * t116945 + 40.0_f64 / 9.0_f64 * t116936 + 80.0_f64 / 27.0_f64 * t116910 - 5.0_f64 / 3.0_f64 * t32245 * t119917 - 5.0_f64 / 9.0_f64 * t116929 * t33111 - 5.0_f64 / 9.0_f64 * t32249 * t119924 - 5.0_f64 / 9.0_f64 * t32249 * t119928 + 10.0_f64 / 9.0_f64 * t119932 * t8706 * t119933 - 5.0_f64 / 3.0_f64 * t32245 * t119948 - 20.0_f64 / 27.0_f64 * t124324 - 10.0_f64 / 9.0_f64 * t116954 * t119902 + 10.0_f64 / 3.0_f64 * t116905 * t119880;
    (t124293, t124330)
}
