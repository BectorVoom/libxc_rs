//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1156/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1156<F: Float>(t19508: F, t19554: F, t19606: F, t20149: F, t1079: F, t20112: F, t225: F, t385: F, t1096: F, t6392: F, t3269: F, t1647: F, t1678: F, t378: F, t6235: F, t1076: F, t1097: F, t11187: F, t16340: F, t16374: F, t1652: F, t16597: F, t1696: F, t19856: F, t3264: F, t342: F, t386: F, t4778: F, t4932: F, t4941: F, t6245: F, t6345: F, t6351: F, t989: F) -> (F,) {
    let t20151 = t19508 + t19554 + t19606 + t20149;
    let t20152 = t1079 * t20151;
    let t20168 = t20112 * t225 * t385;
    let t20171 = t6392 * t1096;
    let t20172 = t3269 * t20171;
    let t20175 = t1647 * t1678;
    let t20178 = t6235 * t378;
    let t20187 = -0.65854491829355115987e0 * t1076 * t20152 + 0.65854491829355115987e0 * t989 * t6345 + 0.13170898365871023197e1 * t1647 * t4932 + 0.13170898365871023197e1 * t4778 * t4941 + 0.13170898365871023197e1 * t3264 * t6351 + 0.65854491829355115987e0 * t19856 * t386 + 0.13170898365871023197e1 * t11187 * t6245 + 0.65854491829355115987e0 * t342 * t20168 + 0.13170898365871023197e1 * t1076 * t20172 - 0.13170898365871023197e1 * t20175 * t1097 - 0.65854491829355115987e0 * t20178 * t1097 - 0.13170898365871023197e1 * t16597 * t1652 - 0.13170898365871023197e1 * t16340 * t1696 - 0.13170898365871023197e1 * t16374 * t1652;
    (t20187,)
}
