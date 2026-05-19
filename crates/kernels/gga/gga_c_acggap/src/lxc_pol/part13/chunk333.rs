//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 333/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk333<F: Float>(t119: F, t1212: F, t1215: F, t1222: F, t1265: F, t446: F, t464: F, t850: F, t854: F, t855: F, t858: F, t867: F, t869: F, t873: F, t882: F) -> F {
    let t1268 = t850 - t854 + F::cast_from(0.13170898365871023197e1_f64) * t855 - F::cast_from(0.13170898365871023197e1_f64) * t858 + t867 - F::cast_from(0.13170898365871023197e1_f64) * t869 + F::cast_from(0.13170898365871023197e1_f64) * t873 - t882 + F::cast_from(0.65854491829355115987e0_f64) * t119 * t1212 - F::cast_from(0.13170898365871023197e1_f64) * t1215 * t464 + F::cast_from(0.13170898365871023197e1_f64) * t446 * t1222 - F::cast_from(0.65854491829355115987e0_f64) * t446 * t1265;
    t1268
}
