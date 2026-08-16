//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1128/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1128(t1928: f64, t980: f64, t12309: f64, t6466: f64, t19718: f64, t3073: f64, t3457: f64, t1410: f64, t157: f64, t4180: f64, t6547: f64, t12395: f64, t12410: f64, t12419: f64, t14442: f64, t14446: f64, t151: f64, t1651: f64, t19074: f64, t19082: f64, t456: f64, t5080: f64) -> (f64, f64, f64) {
    let t20165 = t980 * t1928;
    let t20169 = t12309 * t6466;
    let t20172 = t3073 * t19718 * t3457;
    let t20174 = t1410 * t1410;
    let t20175 = t20174 * t157;
    let t20185 = t4180 * t6547;
    let t20188 = -0.65854491829355115987e0_f64 * t20165 - t12395 + 0.65854491829355115987e0_f64 * t12410 - 0.13170898365871023197e1_f64 * t12419 - 0.26341796731742046394e1_f64 * t20169 - 0.26341796731742046394e1_f64 * t20172 - 0.13170898365871023197e1_f64 * t151 * t456 * t20175 - 0.13170898365871023197e1_f64 * t151 * t1651 * t5080 + 0.79025390195226139182e1_f64 * t19074 + 0.13170898365871023197e1_f64 * t14442 + 0.26341796731742046394e1_f64 * t19082 + 0.52683593463484092788e1_f64 * t20185 - 0.79025390195226139182e1_f64 * t14446;
    (t20174, t20175, t20188)
}
