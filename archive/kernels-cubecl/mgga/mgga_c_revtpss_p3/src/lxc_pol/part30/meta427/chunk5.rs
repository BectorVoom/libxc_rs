//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1633/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1633<F: Float>(t4995: F, t989: F, t1024: F, t1087: F, t1093: F, t11940: F, t12146: F, t15670: F, t15886: F, t16479: F, t16482: F, t16485: F, t16488: F, t16496: F, t16499: F, t16502: F, t16506: F, t16509: F, t16515: F, t16520: F, t3204: F, t3223: F, t3283: F, t3288: F, t3305: F, t3317: F, t381: F, t4743: F, t4967: F, t4977: F, t4984: F, t4999: F) -> F {
    let t16523 = t989 * t4995;
    let t16526 = F::cast_from(0.13170898365871023197e1_f64) * t15670 * t3283 - F::cast_from(0.65854491829355115987e0_f64) * t1024 * t16479 + F::cast_from(0.26341796731742046394e1_f64) * t3204 * t16482 + F::cast_from(0.13170898365871023197e1_f64) * t3204 * t16485 - F::cast_from(0.65854491829355115987e0_f64) * t3317 * t16488 + F::cast_from(0.65854491829355115987e0_f64) * t15886 * t381 + F::cast_from(0.13170898365871023197e1_f64) * t4743 * t1093 + F::cast_from(0.13170898365871023197e1_f64) * t1087 * t16496 - F::cast_from(0.39512695097613069591e1_f64) * t11940 * t16499 - F::cast_from(0.13170898365871023197e1_f64) * t16502 * t3288 - F::cast_from(0.13170898365871023197e1_f64) * t16506 * t4999 + F::cast_from(0.13170898365871023197e1_f64) * t16509 * t3305 - F::cast_from(0.13170898365871023197e1_f64) * t12146 * t4977 + F::cast_from(0.65854491829355115987e0_f64) * t1087 * t16515 - F::cast_from(0.13170898365871023197e1_f64) * t3223 * t4967 + F::cast_from(0.26341796731742046394e1_f64) * t16520 * t4984 - F::cast_from(0.13170898365871023197e1_f64) * t16523 * t4999;
    t16526
}
