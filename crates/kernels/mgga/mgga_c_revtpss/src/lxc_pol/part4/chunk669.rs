//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 669/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk669<F: Float>(t3303: F, t357: F, t3300: F, t3259: F, t380: F, t1024: F, t1083: F, t1087: F, t1090: F, t1093: F, t3043: F, t3204: F, t3223: F, t3278: F, t3283: F, t3287: F, t3288: F, t3292: F, t3295: F, t3299: F, t3305: F, t3309: F, t3313: F, t3317: F, t342: F, t381: F, t989: F) -> (F, F, F, F) {
    let t3318 = t3303 * t357;
    let t3319 = t3300 * t3318;
    let t3322 = t380 * t3259;
    let t3325 = F::cast_from(0.65854491829355115987e0_f64) * t3043 * t381 - F::cast_from(0.13170898365871023197e1_f64) * t3223 * t1083 + F::cast_from(0.13170898365871023197e1_f64) * t3278 * t1090 + F::cast_from(0.13170898365871023197e1_f64) * t989 * t1093 + F::cast_from(0.13170898365871023197e1_f64) * t3204 * t3283 - F::cast_from(0.13170898365871023197e1_f64) * t3287 * t3288 - F::cast_from(0.13170898365871023197e1_f64) * t1024 * t3292 - F::cast_from(0.65854491829355115987e0_f64) * t1024 * t3295 + F::cast_from(0.13170898365871023197e1_f64) * t3299 * t3305 + F::cast_from(0.13170898365871023197e1_f64) * t1087 * t3309 + F::cast_from(0.65854491829355115987e0_f64) * t1087 * t3313 - F::cast_from(0.65854491829355115987e0_f64) * t3317 * t3319 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t3322;
    (t3318, t3319, t3322, t3325)
}
