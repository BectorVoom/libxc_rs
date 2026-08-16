//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 573/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk573<F: Float>(t1605: F, t310: F, t1215: F, t1265: F, t1608: F, t1620: F, t3856: F, t3859: F, t3862: F, t3869: F, t3871: F, t4103: F, t4107: F, t4109: F, t4113: F, t4119: F, t446: F, t464: F) -> F {
    let t4123 = F::cast_from(0.13170898365871023197e1_f64) * t310 * t1605;
    let t4128 = -F::cast_from(0.13170898365871023197e1_f64) * t4103 * t464 + t4107 - F::cast_from(0.39512695097613069591e1_f64) * t446 * t4109 + t4113 - F::cast_from(0.65854491829355115987e0_f64) * t1608 * t1265 - F::cast_from(0.65854491829355115987e0_f64) * t3856 - F::cast_from(0.65854491829355115987e0_f64) * t3859 + F::cast_from(0.26341796731742046394e1_f64) * t446 * t4119 + t3862 + t4123 + F::cast_from(0.26341796731742046394e1_f64) * t3869 + F::cast_from(0.26341796731742046394e1_f64) * t1215 * t1620 + F::cast_from(0.65854491829355115987e0_f64) * t3871;
    t4128
}
