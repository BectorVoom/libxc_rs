//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1163/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1163<F: Float>(t1079: F, t1651: F, t5015: F, t4772: F, t996: F, t16313: F, t4940: F, t6258: F, t999: F, t1096: F, t6244: F, t6350: F, t11121: F, t3268: F, t4946: F, t1076: F, t11224: F, t16284: F, t16312: F, t16333: F, t16371: F, t16603: F, t1696: F, t3047: F, t3058: F, t3063: F, t4747: F, t4758: F, t4935: F, t4941: F, t5016: F, t6245: F, t6251: F, t995: F) -> (F, F, F) {
    let t19396 = t1079 * t1651 * t5015;
    let t19399 = t1651 * t4772;
    let t19400 = t996 * t19399;
    let t19403 = t16313 * t4940;
    let t19414 = t6258 * t999;
    let t19415 = t996 * t19414;
    let t19421 = t1079 * t6244 * t1096;
    let t19424 = t6350 * t1096;
    let t19425 = t11121 * t19424;
    let t19428 = t3268 * t1651;
    let t19429 = t19428 * t4946;
    let t19434 = 0.13170898365871023197e1 * t3063 * t6251 + 0.13170898365871023197e1 * t3047 * t6251 + 0.13170898365871023197e1 * t995 * t19396 + 0.26341796731742046394e1 * t3058 * t19400 - 0.26341796731742046394e1 * t16312 * t19403 - 0.13170898365871023197e1 * t4935 * t5016 + 0.13170898365871023197e1 * t4747 * t4941 - 0.13170898365871023197e1 * t16333 * t1696 - 0.13170898365871023197e1 * t16371 * t1696 + 0.13170898365871023197e1 * t3058 * t19415 + 0.26341796731742046394e1 * t16284 * t4758 - 0.13170898365871023197e1 * t3058 * t19421 - 0.39512695097613069591e1 * t1076 * t19425 - 0.26341796731742046394e1 * t16603 * t19429 + 0.13170898365871023197e1 * t11224 * t6245;
    (t19399, t19414, t19434)
}
