//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1635/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1635(t4995: f64, t989: f64, t1024: f64, t1087: f64, t1093: f64, t11940: f64, t12146: f64, t15670: f64, t15886: f64, t16479: f64, t16482: f64, t16485: f64, t16488: f64, t16496: f64, t16499: f64, t16502: f64, t16506: f64, t16509: f64, t16515: f64, t16520: f64, t3204: f64, t3223: f64, t3283: f64, t3288: f64, t3305: f64, t3317: f64, t381: f64, t4743: f64, t4967: f64, t4977: f64, t4984: f64, t4999: f64) -> f64 {
    let t16523 = t989 * t4995;
    let t16526 = 0.13170898365871023197e1_f64 * t15670 * t3283 - 0.65854491829355115987e0_f64 * t1024 * t16479 + 0.26341796731742046394e1_f64 * t3204 * t16482 + 0.13170898365871023197e1_f64 * t3204 * t16485 - 0.65854491829355115987e0_f64 * t3317 * t16488 + 0.65854491829355115987e0_f64 * t15886 * t381 + 0.13170898365871023197e1_f64 * t4743 * t1093 + 0.13170898365871023197e1_f64 * t1087 * t16496 - 0.39512695097613069591e1_f64 * t11940 * t16499 - 0.13170898365871023197e1_f64 * t16502 * t3288 - 0.13170898365871023197e1_f64 * t16506 * t4999 + 0.13170898365871023197e1_f64 * t16509 * t3305 - 0.13170898365871023197e1_f64 * t12146 * t4977 + 0.65854491829355115987e0_f64 * t1087 * t16515 - 0.13170898365871023197e1_f64 * t3223 * t4967 + 0.26341796731742046394e1_f64 * t16520 * t4984 - 0.13170898365871023197e1_f64 * t16523 * t4999;
    t16526
}
