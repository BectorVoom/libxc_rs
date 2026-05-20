//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1361/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1361<F: Float>(t16604: F, t3066: F, t1000: F, t1076: F, t1097: F, t11128: F, t11210: F, t11214: F, t16362: F, t16371: F, t16374: F, t1652: F, t16592: F, t16597: F, t16600: F, t16603: F, t1696: F, t3047: F, t3060: F, t3067: F, t3076: F, t3264: F, t4747: F, t4773: F, t4778: F, t5016: F) -> F {
    let t16605 = t16604 * t3066;
    let t16610 = -F::cast_from(0.65854491829355115987e0_f64) * t4778 * t3076 - F::cast_from(0.13170898365871023197e1_f64) * t3264 * t5016 - F::cast_from(0.65854491829355115987e0_f64) * t11210 * t1696 - F::cast_from(0.13170898365871023197e1_f64) * t16362 * t1097 - F::cast_from(0.65854491829355115987e0_f64) * t4747 * t3076 - F::cast_from(0.13170898365871023197e1_f64) * t11128 * t1652 - F::cast_from(0.65854491829355115987e0_f64) * t11214 * t1652 - F::cast_from(0.13170898365871023197e1_f64) * t16371 * t1097 - F::cast_from(0.13170898365871023197e1_f64) * t16374 * t1000 - F::cast_from(0.65854491829355115987e0_f64) * t1076 * t16592 - F::cast_from(0.13170898365871023197e1_f64) * t3047 * t4773 - F::cast_from(0.13170898365871023197e1_f64) * t16597 * t1000 + F::cast_from(0.13170898365871023197e1_f64) * t16600 * t3060 - F::cast_from(0.26341796731742046394e1_f64) * t16603 * t16605 + F::cast_from(0.13170898365871023197e1_f64) * t4747 * t3067;
    t16610
}
