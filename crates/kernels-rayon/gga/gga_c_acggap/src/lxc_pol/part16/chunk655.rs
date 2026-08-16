//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 655/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk655(t1215: f64, t1608: f64, t1620: f64, t1938: f64, t3906: f64, t3914: f64, t3917: f64, t3920: f64, t3925: f64, t5346: f64, t5352: f64, t5354: f64, t5359: f64, t5361: f64, t5364: f64) -> f64 {
    let t6453 = 0.26341796731742046394e1_f64 * t1608 * t1620 + 0.13170898365871023197e1_f64 * t5346 + 0.13170898365871023197e1_f64 * t3906 - t3914 - 0.26341796731742046394e1_f64 * t5352 + 0.26341796731742046394e1_f64 * t5354 + t3917 - 0.13170898365871023197e1_f64 * t3920 - t3925 - 0.65854491829355115987e0_f64 * t1215 * t1938 - t5359 - 0.26341796731742046394e1_f64 * t5361 + t5364;
    t6453
}
