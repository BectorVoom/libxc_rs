//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1353/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1353(t15957: f64, t4976: f64, t1024: f64, t1087: f64, t11782: f64, t11788: f64, t12122: f64, t12127: f64, t12149: f64, t16427: f64, t16433: f64, t16436: f64, t16440: f64, t16443: f64, t16446: f64, t16450: f64, t16458: f64, t16461: f64, t16465: f64, t1685: f64, t1692: f64, t3043: f64, t3223: f64, t3278: f64, t3287: f64, t3299: f64, t3313: f64, t4954: f64, t4961: f64, t4981: f64, t4988: f64, t5005: f64) -> f64 {
    let t16468 = t15957 * t4976;
    let t16475 = 0.65854491829355115987e0_f64 * t4954 * t3313 + 0.13170898365871023197e1_f64 * t3299 * t16427 + 0.26341796731742046394e1_f64 * t11788 * t4961 - 0.26341796731742046394e1_f64 * t12122 * t16433 + 0.13170898365871023197e1_f64 * t12127 * t16436 + 0.65854491829355115987e0_f64 * t1087 * t16440 + 0.26341796731742046394e1_f64 * t4981 * t16443 - 0.65854491829355115987e0_f64 * t1024 * t16446 - 0.13170898365871023197e1_f64 * t1024 * t16450 - 0.65854491829355115987e0_f64 * t11782 * t1685 + 0.65854491829355115987e0_f64 * t3043 * t1692 + 0.26341796731742046394e1_f64 * t12149 * t16458 - 0.13170898365871023197e1_f64 * t1024 * t16461 + 0.65854491829355115987e0_f64 * t1087 * t16465 - 0.13170898365871023197e1_f64 * t3287 * t16468 + 0.13170898365871023197e1_f64 * t3278 * t4988 - 0.13170898365871023197e1_f64 * t3223 * t5005;
    t16475
}
