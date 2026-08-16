//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 571/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk571(t1605: f64, t310: f64, t1215: f64, t1265: f64, t1608: f64, t1620: f64, t3856: f64, t3859: f64, t3862: f64, t3869: f64, t3871: f64, t4103: f64, t4107: f64, t4109: f64, t4113: f64, t4119: f64, t446: f64, t464: f64) -> f64 {
    let t4123 = 0.13170898365871023197e1_f64 * t310 * t1605;
    let t4128 = -0.13170898365871023197e1_f64 * t4103 * t464 + t4107 - 0.39512695097613069591e1_f64 * t446 * t4109 + t4113 - 0.65854491829355115987e0_f64 * t1608 * t1265 - 0.65854491829355115987e0_f64 * t3856 - 0.65854491829355115987e0_f64 * t3859 + 0.26341796731742046394e1_f64 * t446 * t4119 + t3862 + t4123 + 0.26341796731742046394e1_f64 * t3869 + 0.26341796731742046394e1_f64 * t1215 * t1620 + 0.65854491829355115987e0_f64 * t3871;
    t4128
}
