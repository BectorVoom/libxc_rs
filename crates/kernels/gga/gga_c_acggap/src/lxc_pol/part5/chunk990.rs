//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 990/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk990<F: Float>(t1160: F, t4162: F, t6465: F, t12285: F, t12286: F, t12290: F, t1530: F, t18880: F, t18884: F, t18887: F, t18890: F, t18893: F, t19716: F, t19718: F, t3084: F, t4166: F, t4198: F, t5853: F, t6263: F, t6482: F) -> (F,) {
    let t19732 = t1160 * t6465 * t4162;
    let t19738 = -t12285 - 0.79025390195226139182e1 * t19716 - 0.79025390195226139182e1 * t4198 * t19718 * t5853 + 0.26341796731742046394e1 * t12286 + 0.13170898365871023197e1 * t12290 - 0.52683593463484092788e1 * t18880 - 0.52683593463484092788e1 * t18884 - 0.52683593463484092788e1 * t18887 - 0.26341796731742046394e1 * t18890 + 0.52683593463484092788e1 * t1530 * t4166 * t6263 + 0.65854491829355115987e0 * t19732 + 0.13170898365871023197e1 * t1530 * t6482 * t3084 + 0.79025390195226139182e1 * t18893;
    (t19738,)
}
