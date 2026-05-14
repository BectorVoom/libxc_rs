//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 982/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk982<F: Float>(t6434: F, t872: F, t1614: F, t5360: F, t12225: F, t12229: F, t12233: F, t12238: F, t12240: F, t12241: F, t12243: F, t12246: F, t15149: F, t15152: F, t15154: F, t15156: F, t15159: F, t15164: F, t15175: F, t15177: F, t15179: F, t1620: F, t4103: F) -> (F,) {
    let t19545 = t6434 * t872;
    let t19550 = t5360 * t1614;
    let t19565 = -0.13170898365871023197e1 * t12225 + 0.13170898365871023197e1 * t19545 + 0.26341796731742046394e1 * t15149 + 0.52683593463484092788e1 * t4103 * t1620 + 0.26341796731742046394e1 * t19550 - 0.26341796731742046394e1 * t15152 + 0.52683593463484092788e1 * t15154 - 0.52683593463484092788e1 * t15156 + 0.79025390195226139182e1 * t12229 + 0.79025390195226139182e1 * t15159 + 0.26341796731742046394e1 * t12233 - t12238 + t12240 - 0.79025390195226139182e1 * t15164 + 0.39512695097613069592e1 * t12241 - 0.79025390195226139182e1 * t12243 - 0.26341796731742046394e1 * t12246 + 0.52683593463484092788e1 * t15175 + 0.26341796731742046394e1 * t15177 - 0.13170898365871023197e1 * t15179;
    (t19565,)
}
