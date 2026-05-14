//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1179/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1179<F: Float>(t16505: F, t342: F, t1647: F, t3298: F, t1089: F, t16183: F, t378: F, t4980: F, t989: F, t4995: F, t1024: F, t1087: F, t1093: F, t11940: F, t12146: F, t15670: F, t15886: F, t16479: F, t16482: F, t16485: F, t16488: F, t16496: F, t16499: F, t16502: F, t3204: F, t3223: F, t3283: F, t3288: F, t3305: F, t3317: F, t381: F, t4743: F, t4967: F, t4977: F, t4984: F, t4999: F) -> (F,) {
    let t16506 = t342 * t16505;
    let t16509 = t1647 * t3298;
    let t16515 = t378 * t16183 * t1089;
    let t16520 = t989 * t4980;
    let t16523 = t989 * t4995;
    let t16526 = 0.13170898365871023197e1 * t15670 * t3283 - 0.65854491829355115987e0 * t1024 * t16479 + 0.26341796731742046394e1 * t3204 * t16482 + 0.13170898365871023197e1 * t3204 * t16485 - 0.65854491829355115987e0 * t3317 * t16488 + 0.65854491829355115987e0 * t15886 * t381 + 0.13170898365871023197e1 * t4743 * t1093 + 0.13170898365871023197e1 * t1087 * t16496 - 0.39512695097613069591e1 * t11940 * t16499 - 0.13170898365871023197e1 * t16502 * t3288 - 0.13170898365871023197e1 * t16506 * t4999 + 0.13170898365871023197e1 * t16509 * t3305 - 0.13170898365871023197e1 * t12146 * t4977 + 0.65854491829355115987e0 * t1087 * t16515 - 0.13170898365871023197e1 * t3223 * t4967 + 0.26341796731742046394e1 * t16520 * t4984 - 0.13170898365871023197e1 * t16523 * t4999;
    (t16526,)
}
