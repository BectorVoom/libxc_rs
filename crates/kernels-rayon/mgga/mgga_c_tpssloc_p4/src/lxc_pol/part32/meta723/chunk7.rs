//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2313/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2313(t24574: f64, t29777: f64, t29678: f64, t7359: f64, t29759: f64, t1244: f64, t1246: f64, t15245: f64, t1734: f64, t19120: f64, t19169: f64, t2121: f64, t2147: f64, t24776: f64, t24858: f64, t27406: f64, t27546: f64, t27574: f64, t27721: f64, t29711: f64, t3624: f64, t462: f64, t5079: f64, t5971: f64, t7283: f64, t7373: f64, t7375: f64, t7376: f64, t95714: f64, t95722: f64) -> f64 {
    let t103927 = t24574 * t29777;
    let t103939 = t29678 * t7359;
    let t103943 = t24574 * t29759;
    let t103949 = 0.43864908449286038306e-1_f64 * t27406 * t27574 + 0.36554090374405031923e-2_f64 * t7283 * t24776 * t24858 * t5971 + 0.12184696791468343974e-2_f64 * t103927 + 0.16449340668482264365e-1_f64 * t7373 * t7375 * t19169 * t7376 + 0.82246703342411321825e-2_f64 * t2121 * t462 * t2147 * t19120 - t95714 - t3624 * t29711 * t5079 + 0.26806332941230356743e-1_f64 * t103939 - 2.0_f64 * t15245 * t27546 - 0.91385225936012579807e-3_f64 * t103943 + 2.0_f64 * t1244 * t27721 * t1734 * t1246 - t95722;
    t103949
}
