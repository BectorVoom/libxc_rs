//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2159/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2159(t1082: f64, t15837: f64, t3075: f64, t4975: f64, t4781: f64, t1071: f64, t3298: f64, t342: f64, t1089: f64, t4866: f64, t1024: f64, t1087: f64, t1090: f64, t12097: f64, t12154: f64, t16381: f64, t16390: f64, t16393: f64, t16396: f64, t16399: f64, t1647: f64, t1689: f64, t3204: f64, t3223: f64, t3278: f64, t3287: f64, t3292: f64, t3295: f64, t3322: f64, t4857: f64, t4964: f64, t4970: f64, t4984: f64, t4992: f64, t5012: f64, t989: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16402 = t1082 * t15837;
    let t16405 = t4975 * t3075;
    let t16406 = t4781 * t16405;
    let t16409 = t3298 * t1071;
    let t16410 = t342 * t16409;
    let t16414 = t1071 * t4866 * t1089;
    let t16423 = -0.65854491829355115987e0_f64 * t4857 * t3295 + 0.13170898365871023197e1_f64 * t989 * t5012 + 0.13170898365871023197e1_f64 * t16381 * t1090 + 0.13170898365871023197e1_f64 * t3278 * t4992 + 0.65854491829355115987e0_f64 * t1647 * t3322 - 0.13170898365871023197e1_f64 * t12154 * t4964 - 0.13170898365871023197e1_f64 * t3287 * t16390 - 0.65854491829355115987e0_f64 * t3287 * t16393 - 0.65854491829355115987e0_f64 * t1024 * t16396 + 0.26341796731742046394e1_f64 * t3204 * t16399 + 0.13170898365871023197e1_f64 * t3204 * t16402 - 0.65854491829355115987e0_f64 * t3287 * t16406 + 0.26341796731742046394e1_f64 * t16410 * t4984 + 0.13170898365871023197e1_f64 * t1087 * t16414 - 0.13170898365871023197e1_f64 * t4857 * t3292 - 0.13170898365871023197e1_f64 * t3223 * t4970 + 0.65854491829355115987e0_f64 * t12097 * t1689;
    (t16402, t16405, t16406, t16409, t16410, t16414, t16423)
}
