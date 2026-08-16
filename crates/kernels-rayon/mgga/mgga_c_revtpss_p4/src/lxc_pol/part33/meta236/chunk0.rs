//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1062/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1062(t378: f64, t6305: f64, t3304: f64, t1089: f64, t1668: f64, t1678: f64, t6299: f64, t3318: f64, t380: f64, t6343: f64, t1024: f64, t1087: f64, t1647: f64, t1685: f64, t1689: f64, t1692: f64, t3204: f64, t3287: f64, t3299: f64, t3317: f64, t342: f64, t381: f64, t4857: f64, t4954: f64, t6235: f64, t6362: f64, t6365: f64, t6368: f64, t6371: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6374 = t378 * t6305;
    let t6375 = t6374 * t3304;
    let t6379 = t1678 * t1668 * t1089;
    let t6383 = t378 * t6299 * t1089;
    let t6386 = t6374 * t3318;
    let t6389 = t380 * t6343;
    let t6392 = 0.65854491829355115987e0_f64 * t6235 * t381 - 0.13170898365871023197e1_f64 * t4857 * t1685 + 0.13170898365871023197e1_f64 * t4954 * t1689 + 0.13170898365871023197e1_f64 * t1647 * t1692 + 0.13170898365871023197e1_f64 * t3204 * t6362 - 0.13170898365871023197e1_f64 * t3287 * t6365 - 0.13170898365871023197e1_f64 * t1024 * t6368 - 0.65854491829355115987e0_f64 * t1024 * t6371 + 0.13170898365871023197e1_f64 * t3299 * t6375 + 0.13170898365871023197e1_f64 * t1087 * t6379 + 0.65854491829355115987e0_f64 * t1087 * t6383 - 0.65854491829355115987e0_f64 * t3317 * t6386 + 0.65854491829355115987e0_f64 * t342 * t6389;
    (t6375, t6379, t6383, t6386, t6389, t6392)
}
