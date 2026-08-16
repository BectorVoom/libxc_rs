//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1638/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1638(t16604: f64, t3066: f64, t1000: f64, t1076: f64, t1097: f64, t11128: f64, t11210: f64, t11214: f64, t16362: f64, t16371: f64, t16374: f64, t1652: f64, t16592: f64, t16597: f64, t16600: f64, t16603: f64, t1696: f64, t3047: f64, t3060: f64, t3067: f64, t3076: f64, t3264: f64, t4747: f64, t4773: f64, t4778: f64, t5016: f64) -> f64 {
    let t16605 = t16604 * t3066;
    let t16610 = -0.65854491829355115987e0_f64 * t4778 * t3076 - 0.13170898365871023197e1_f64 * t3264 * t5016 - 0.65854491829355115987e0_f64 * t11210 * t1696 - 0.13170898365871023197e1_f64 * t16362 * t1097 - 0.65854491829355115987e0_f64 * t4747 * t3076 - 0.13170898365871023197e1_f64 * t11128 * t1652 - 0.65854491829355115987e0_f64 * t11214 * t1652 - 0.13170898365871023197e1_f64 * t16371 * t1097 - 0.13170898365871023197e1_f64 * t16374 * t1000 - 0.65854491829355115987e0_f64 * t1076 * t16592 - 0.13170898365871023197e1_f64 * t3047 * t4773 - 0.13170898365871023197e1_f64 * t16597 * t1000 + 0.13170898365871023197e1_f64 * t16600 * t3060 - 0.26341796731742046394e1_f64 * t16603 * t16605 + 0.13170898365871023197e1_f64 * t4747 * t3067;
    t16610
}
