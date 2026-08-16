//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1308/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1308(t1089: f64, t19829: f64, t19836: f64, t1024: f64, t1087: f64, t12146: f64, t12149: f64, t12154: f64, t15670: f64, t19608: f64, t19612: f64, t19617: f64, t19856: f64, t20113: f64, t20119: f64, t20123: f64, t20128: f64, t20133: f64, t20136: f64, t3204: f64, t3278: f64, t3287: f64, t342: f64, t381: f64, t4961: f64, t4999: f64, t6365: f64, t6379: f64, t6389: f64, t989: f64) -> f64 {
    let t20139 = t19829 * t1089;
    let t20146 = t19836 * t1089;
    let t20149 = -0.13170898365871023197e1_f64 * t19608 * t4999 - 0.65854491829355115987e0_f64 * t3287 * t19612 + 0.65854491829355115987e0_f64 * t989 * t6389 + 0.13170898365871023197e1_f64 * t3204 * t19617 + 0.65854491829355115987e0_f64 * t342 * t20113 + 0.65854491829355115987e0_f64 * t19856 * t381 + 0.65854491829355115987e0_f64 * t1087 * t20119 + 0.13170898365871023197e1_f64 * t1087 * t20123 + 0.13170898365871023197e1_f64 * t3278 * t6379 - 0.13170898365871023197e1_f64 * t1024 * t20128 + 0.26341796731742046394e1_f64 * t15670 * t4961 - 0.13170898365871023197e1_f64 * t1024 * t20133 - 0.13170898365871023197e1_f64 * t3287 * t20136 + 0.13170898365871023197e1_f64 * t12149 * t20139 - 0.13170898365871023197e1_f64 * t12146 * t6365 - 0.13170898365871023197e1_f64 * t12154 * t6365 - 0.13170898365871023197e1_f64 * t3287 * t20146;
    t20149
}
