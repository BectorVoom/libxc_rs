//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1304/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1304<F: Float>(t1082: F, t15837: F, t3075: F, t4975: F, t4781: F, t1071: F, t3298: F, t342: F, t1089: F, t4866: F, t1024: F, t1087: F, t1090: F, t12097: F, t12154: F, t16381: F, t16390: F, t16393: F, t16396: F, t16399: F, t1647: F, t1689: F, t3204: F, t3223: F, t3278: F, t3287: F, t3292: F, t3295: F, t3322: F, t4857: F, t4964: F, t4970: F, t4984: F, t4992: F, t5012: F, t989: F) -> F {
    let t16402 = t1082 * t15837;
    let t16405 = t4975 * t3075;
    let t16406 = t4781 * t16405;
    let t16409 = t3298 * t1071;
    let t16410 = t342 * t16409;
    let t16414 = t1071 * t4866 * t1089;
    let t16423 = -F::new(0.65854491829355115987e0) * t4857 * t3295 + F::new(0.13170898365871023197e1) * t989 * t5012 + F::new(0.13170898365871023197e1) * t16381 * t1090 + F::new(0.13170898365871023197e1) * t3278 * t4992 + F::new(0.65854491829355115987e0) * t1647 * t3322 - F::new(0.13170898365871023197e1) * t12154 * t4964 - F::new(0.13170898365871023197e1) * t3287 * t16390 - F::new(0.65854491829355115987e0) * t3287 * t16393 - F::new(0.65854491829355115987e0) * t1024 * t16396 + F::new(0.26341796731742046394e1) * t3204 * t16399 + F::new(0.13170898365871023197e1) * t3204 * t16402 - F::new(0.65854491829355115987e0) * t3287 * t16406 + F::new(0.26341796731742046394e1) * t16410 * t4984 + F::new(0.13170898365871023197e1) * t1087 * t16414 - F::new(0.13170898365871023197e1) * t4857 * t3292 - F::new(0.13170898365871023197e1) * t3223 * t4970 + F::new(0.65854491829355115987e0) * t12097 * t1689;
    t16423
}
