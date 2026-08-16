//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 544/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk544(t3303: f64, t357: f64, t3300: f64, t3259: f64, t380: f64, t1024: f64, t1083: f64, t1087: f64, t1090: f64, t1093: f64, t3043: f64, t3204: f64, t3223: f64, t3278: f64, t3283: f64, t3287: f64, t3288: f64, t3292: f64, t3295: f64, t3299: f64, t3305: f64, t3309: f64, t3313: f64, t3317: f64, t342: f64, t381: f64, t989: f64) -> (f64, f64, f64, f64) {
    let t3318 = t3303 * t357;
    let t3319 = t3300 * t3318;
    let t3322 = t380 * t3259;
    let t3325 = 0.65854491829355115987e0_f64 * t3043 * t381 - 0.13170898365871023197e1_f64 * t3223 * t1083 + 0.13170898365871023197e1_f64 * t3278 * t1090 + 0.13170898365871023197e1_f64 * t989 * t1093 + 0.13170898365871023197e1_f64 * t3204 * t3283 - 0.13170898365871023197e1_f64 * t3287 * t3288 - 0.13170898365871023197e1_f64 * t1024 * t3292 - 0.65854491829355115987e0_f64 * t1024 * t3295 + 0.13170898365871023197e1_f64 * t3299 * t3305 + 0.13170898365871023197e1_f64 * t1087 * t3309 + 0.65854491829355115987e0_f64 * t1087 * t3313 - 0.65854491829355115987e0_f64 * t3317 * t3319 + 0.65854491829355115987e0_f64 * t342 * t3322;
    (t3318, t3319, t3322, t3325)
}
