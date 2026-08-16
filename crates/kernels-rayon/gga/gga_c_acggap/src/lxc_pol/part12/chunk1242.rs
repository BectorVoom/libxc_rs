//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1242/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1242(t32130: f64, t38052: f64, t7965: f64, t2387: f64, t848: f64, t5351: f64, t8347: f64, t2146: f64, t2147: f64, t33256: f64, t33258: f64, t33262: f64, t38377: f64, t38379: f64, t38382: f64, t38386: f64, t38389: f64, t5331: f64, t633: f64, t8108: f64, t8126: f64, t9003: f64) -> f64 {
    let t38392 = 0.34694512752820797848e1_f64 * t32130 * t38052 * t7965;
    let t38393 = t848 * t2387;
    let t38397 = t8347 * t5351;
    let t38406 = -0.26020884564615598386e1_f64 * t9003 * t8108 + t38377 - t38379 - t38382 - t38386 + t38389 - t38392 + 0.65854491829355115987e0_f64 * t38393 - 0.17347256376410398924e1_f64 * t33256 + 0.13170898365871023197e1_f64 * t33258 - 0.13170898365871023197e1_f64 * t38397 + 0.8673628188205199462e0_f64 * t33262 + 0.8673628188205199462e0_f64 * t2146 * t2147 * t633 * t5331 + 0.8673628188205199462e0_f64 * t9003 * t8126;
    t38406
}
