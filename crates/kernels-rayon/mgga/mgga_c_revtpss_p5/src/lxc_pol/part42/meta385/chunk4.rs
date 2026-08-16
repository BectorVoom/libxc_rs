//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1274/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1274(t19533: f64, t3318: f64, t3304: f64, t1043: f64, t16553: f64, t19450: f64, t1093: f64, t11788: f64, t12160: f64, t15655: f64, t16502: f64, t16544: f64, t16552: f64, t1685: f64, t19509: f64, t19512: f64, t19515: f64, t19521: f64, t19526: f64, t3204: f64, t3223: f64, t3299: f64, t3317: f64, t4857: f64, t4964: f64, t4967: f64, t4977: f64, t4981: f64, t4984: f64, t6235: f64, t6362: f64, t6371: f64, t6386: f64) -> f64 {
    let t19534 = t19533 * t3318;
    let t19539 = t19533 * t3304;
    let t19548 = t16553 * t1043;
    let t19549 = t19450 * t19548;
    let t19554 = 0.26341796731742046394e1_f64 * t3204 * t19509 + 0.13170898365871023197e1_f64 * t3204 * t19512 + 0.26341796731742046394e1_f64 * t3204 * t19515 + 0.13170898365871023197e1_f64 * t11788 * t6362 + 0.26341796731742046394e1_f64 * t4981 * t19521 - 0.13170898365871023197e1_f64 * t16544 * t4964 + 0.26341796731742046394e1_f64 * t19526 * t4984 - 0.13170898365871023197e1_f64 * t4857 * t4967 - 0.13170898365871023197e1_f64 * t15655 * t1685 - 0.65854491829355115987e0_f64 * t3317 * t19534 - 0.65854491829355115987e0_f64 * t12160 * t6386 + 0.13170898365871023197e1_f64 * t3299 * t19539 - 0.65854491829355115987e0_f64 * t3223 * t6371 - 0.13170898365871023197e1_f64 * t16502 * t4964 - 0.13170898365871023197e1_f64 * t16502 * t4977 + 0.39512695097613069591e1_f64 * t16552 * t19549 + 0.65854491829355115987e0_f64 * t6235 * t1093;
    t19554
}
