//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1538/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1538(t11121: f64, t19424: f64, t1651: f64, t3268: f64, t4946: f64, t1076: f64, t11224: f64, t16284: f64, t16312: f64, t16333: f64, t16371: f64, t16603: f64, t1696: f64, t19396: f64, t19400: f64, t19403: f64, t19415: f64, t19421: f64, t3047: f64, t3058: f64, t3063: f64, t4747: f64, t4758: f64, t4935: f64, t4941: f64, t5016: f64, t6245: f64, t6251: f64, t995: f64) -> (f64, f64, f64) {
    let t19425 = t11121 * t19424;
    let t19428 = t3268 * t1651;
    let t19429 = t19428 * t4946;
    let t19434 = 0.13170898365871023197e1_f64 * t3063 * t6251 + 0.13170898365871023197e1_f64 * t3047 * t6251 + 0.13170898365871023197e1_f64 * t995 * t19396 + 0.26341796731742046394e1_f64 * t3058 * t19400 - 0.26341796731742046394e1_f64 * t16312 * t19403 - 0.13170898365871023197e1_f64 * t4935 * t5016 + 0.13170898365871023197e1_f64 * t4747 * t4941 - 0.13170898365871023197e1_f64 * t16333 * t1696 - 0.13170898365871023197e1_f64 * t16371 * t1696 + 0.13170898365871023197e1_f64 * t3058 * t19415 + 0.26341796731742046394e1_f64 * t16284 * t4758 - 0.13170898365871023197e1_f64 * t3058 * t19421 - 0.39512695097613069591e1_f64 * t1076 * t19425 - 0.26341796731742046394e1_f64 * t16603 * t19429 + 0.13170898365871023197e1_f64 * t11224 * t6245;
    (t19425, t19429, t19434)
}
