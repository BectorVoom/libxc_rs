//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1128/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1128<F: Float>(t1928: F, t980: F, t12309: F, t6466: F, t19718: F, t3073: F, t3457: F, t1410: F, t157: F, t4180: F, t6547: F, t12395: F, t12410: F, t12419: F, t14442: F, t14446: F, t151: F, t1651: F, t19074: F, t19082: F, t456: F, t5080: F) -> (F, F, F) {
    let t20165 = t980 * t1928;
    let t20169 = t12309 * t6466;
    let t20172 = t3073 * t19718 * t3457;
    let t20174 = t1410 * t1410;
    let t20175 = t20174 * t157;
    let t20185 = t4180 * t6547;
    let t20188 = -F::new(0.65854491829355115987e0) * t20165 - t12395 + F::new(0.65854491829355115987e0) * t12410 - F::new(0.13170898365871023197e1) * t12419 - F::new(0.26341796731742046394e1) * t20169 - F::new(0.26341796731742046394e1) * t20172 - F::new(0.13170898365871023197e1) * t151 * t456 * t20175 - F::new(0.13170898365871023197e1) * t151 * t1651 * t5080 + F::new(0.79025390195226139182e1) * t19074 + F::new(0.13170898365871023197e1) * t14442 + F::new(0.26341796731742046394e1) * t19082 + F::new(0.52683593463484092788e1) * t20185 - F::new(0.79025390195226139182e1) * t14446;
    (t20174, t20175, t20188)
}
