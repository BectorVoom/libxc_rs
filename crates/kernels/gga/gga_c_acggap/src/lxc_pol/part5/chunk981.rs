//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 981/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk981<F: Float>(t315: F, t323: F, t6413: F, t1937: F, t316: F, t449: F, t879: F, t6425: F, t857: F, t12196: F, t12198: F, t12201: F, t12206: F, t12208: F, t12212: F, t12218: F, t15106: F, t15110: F, t15112: F, t15116: F, t15126: F, t15129: F, t15132: F, t15135: F, t15138: F, t15297: F, t19278: F, t4118: F) -> (F,) {
    let t19523 = t315 * t6413 * t323;
    let t19527 = t316 * t449 * t1937 * t879;
    let t19540 = t857 * t6425;
    let t19543 = 0.26341796731742046394e1 * t12196 - 0.26341796731742046394e1 * t12198 - 0.13170898365871023197e1 * t12201 - 0.79025390195226139182e1 * t15106 + 0.79025390195226139182e1 * t15110 + 0.26341796731742046394e1 * t15112 - 0.79025390195226139182e1 * t12206 - 0.13170898365871023197e1 * t19523 + 0.65854491829355115987e0 * t19527 - 0.26341796731742046394e1 * t15116 + 0.79025390195226139182e1 * t12208 - 0.13170898365871023197e1 * t12212 + 0.13170898365871023197e1 * t12218 - 0.15805078039045227836e2 * t19278 * t15297 * t4118 - 0.52683593463484092788e1 * t15126 - 0.52683593463484092788e1 * t15129 - 0.52683593463484092788e1 * t15132 + 0.26341796731742046394e1 * t15135 + 0.52683593463484092788e1 * t19540 - 0.10536718692696818558e2 * t15138;
    (t19543,)
}
