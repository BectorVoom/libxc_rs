//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1585/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1585(t378: f64, t6235: f64, t1076: f64, t1097: f64, t11187: f64, t16340: f64, t16374: f64, t1647: f64, t1652: f64, t16597: f64, t1696: f64, t19856: f64, t20152: f64, t20168: f64, t20172: f64, t20175: f64, t3264: f64, t342: f64, t386: f64, t4778: f64, t4932: f64, t4941: f64, t6245: f64, t6345: f64, t6351: f64, t989: f64) -> f64 {
    let t20178 = t6235 * t378;
    let t20187 = -0.65854491829355115987e0_f64 * t1076 * t20152 + 0.65854491829355115987e0_f64 * t989 * t6345 + 0.13170898365871023197e1_f64 * t1647 * t4932 + 0.13170898365871023197e1_f64 * t4778 * t4941 + 0.13170898365871023197e1_f64 * t3264 * t6351 + 0.65854491829355115987e0_f64 * t19856 * t386 + 0.13170898365871023197e1_f64 * t11187 * t6245 + 0.65854491829355115987e0_f64 * t342 * t20168 + 0.13170898365871023197e1_f64 * t1076 * t20172 - 0.13170898365871023197e1_f64 * t20175 * t1097 - 0.65854491829355115987e0_f64 * t20178 * t1097 - 0.13170898365871023197e1_f64 * t16597 * t1652 - 0.13170898365871023197e1_f64 * t16340 * t1696 - 0.13170898365871023197e1_f64 * t16374 * t1652;
    t20187
}
