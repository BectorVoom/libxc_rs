//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1266/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1266<F: Float>(t378: F, t6235: F, t1076: F, t1097: F, t11187: F, t16340: F, t16374: F, t1647: F, t1652: F, t16597: F, t1696: F, t19856: F, t20152: F, t20168: F, t20172: F, t20175: F, t3264: F, t342: F, t386: F, t4778: F, t4932: F, t4941: F, t6245: F, t6345: F, t6351: F, t989: F) -> F {
    let t20178 = t6235 * t378;
    let t20187 = -F::cast_from(0.65854491829355115987e0_f64) * t1076 * t20152 + F::cast_from(0.65854491829355115987e0_f64) * t989 * t6345 + F::cast_from(0.13170898365871023197e1_f64) * t1647 * t4932 + F::cast_from(0.13170898365871023197e1_f64) * t4778 * t4941 + F::cast_from(0.13170898365871023197e1_f64) * t3264 * t6351 + F::cast_from(0.65854491829355115987e0_f64) * t19856 * t386 + F::cast_from(0.13170898365871023197e1_f64) * t11187 * t6245 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t20168 + F::cast_from(0.13170898365871023197e1_f64) * t1076 * t20172 - F::cast_from(0.13170898365871023197e1_f64) * t20175 * t1097 - F::cast_from(0.65854491829355115987e0_f64) * t20178 * t1097 - F::cast_from(0.13170898365871023197e1_f64) * t16597 * t1652 - F::cast_from(0.13170898365871023197e1_f64) * t16340 * t1696 - F::cast_from(0.13170898365871023197e1_f64) * t16374 * t1652;
    t20187
}
