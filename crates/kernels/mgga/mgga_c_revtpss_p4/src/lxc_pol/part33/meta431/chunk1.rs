//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1545/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1545<F: Float>(t11121: F, t19424: F, t1651: F, t3268: F, t4946: F, t1076: F, t11224: F, t16284: F, t16312: F, t16333: F, t16371: F, t16603: F, t1696: F, t19396: F, t19400: F, t19403: F, t19415: F, t19421: F, t3047: F, t3058: F, t3063: F, t4747: F, t4758: F, t4935: F, t4941: F, t5016: F, t6245: F, t6251: F, t995: F) -> F {
    let t19425 = t11121 * t19424;
    let t19428 = t3268 * t1651;
    let t19429 = t19428 * t4946;
    let t19434 = F::cast_from(0.13170898365871023197e1_f64) * t3063 * t6251 + F::cast_from(0.13170898365871023197e1_f64) * t3047 * t6251 + F::cast_from(0.13170898365871023197e1_f64) * t995 * t19396 + F::cast_from(0.26341796731742046394e1_f64) * t3058 * t19400 - F::cast_from(0.26341796731742046394e1_f64) * t16312 * t19403 - F::cast_from(0.13170898365871023197e1_f64) * t4935 * t5016 + F::cast_from(0.13170898365871023197e1_f64) * t4747 * t4941 - F::cast_from(0.13170898365871023197e1_f64) * t16333 * t1696 - F::cast_from(0.13170898365871023197e1_f64) * t16371 * t1696 + F::cast_from(0.13170898365871023197e1_f64) * t3058 * t19415 + F::cast_from(0.26341796731742046394e1_f64) * t16284 * t4758 - F::cast_from(0.13170898365871023197e1_f64) * t3058 * t19421 - F::cast_from(0.39512695097613069591e1_f64) * t1076 * t19425 - F::cast_from(0.26341796731742046394e1_f64) * t16603 * t19429 + F::cast_from(0.13170898365871023197e1_f64) * t11224 * t6245;
    t19434
}
