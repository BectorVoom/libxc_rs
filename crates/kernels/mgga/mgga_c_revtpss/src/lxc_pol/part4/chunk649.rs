//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 649/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk649<F: Float>(t3303: F, t357: F, t3300: F, t3259: F, t380: F, t1024: F, t1083: F, t1087: F, t1090: F, t1093: F, t3043: F, t3204: F, t3223: F, t3278: F, t3283: F, t3287: F, t3288: F, t3292: F, t3295: F, t3299: F, t3305: F, t3309: F, t3313: F, t3317: F, t342: F, t381: F, t989: F) -> (F, F, F, F) {
    let t3318 = t3303 * t357;
    let t3319 = t3300 * t3318;
    let t3322 = t380 * t3259;
    let t3325 = 0.65854491829355115987e0 * t3043 * t381 - 0.13170898365871023197e1 * t3223 * t1083 + 0.13170898365871023197e1 * t3278 * t1090 + 0.13170898365871023197e1 * t989 * t1093 + 0.13170898365871023197e1 * t3204 * t3283 - 0.13170898365871023197e1 * t3287 * t3288 - 0.13170898365871023197e1 * t1024 * t3292 - 0.65854491829355115987e0 * t1024 * t3295 + 0.13170898365871023197e1 * t3299 * t3305 + 0.13170898365871023197e1 * t1087 * t3309 + 0.65854491829355115987e0 * t1087 * t3313 - 0.65854491829355115987e0 * t3317 * t3319 + 0.65854491829355115987e0 * t342 * t3322;
    (t3318, t3319, t3322, t3325)
}
