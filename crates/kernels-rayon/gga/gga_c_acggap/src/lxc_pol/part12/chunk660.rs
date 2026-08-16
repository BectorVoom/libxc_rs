//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 660/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk660(t1220: f64, t1264: f64, t556: f64, t547: f64, t848: f64, t449: f64, t864: f64, t863: f64, t1614: f64, t852: f64, t1222: f64, t1608: f64, t3902: f64, t3904: f64, t3906: f64, t3910: f64, t3914: f64, t3917: f64, t3920: f64, t3925: f64, t446: f64) -> (f64, f64, f64) {
    let t5340 = t1220 * t556 * t1264;
    let t5346 = t848 * t547;
    let t5351 = t449 * t556 * t864;
    let t5352 = t863 * t5351;
    let t5354 = t852 * t1614;
    let t5357 = -0.26341796731742046395e1_f64 * t3902 + 0.13170898365871023197e1_f64 * t446 * t5340 + 0.13170898365871023197e1_f64 * t1608 * t1222 + 0.13170898365871023197e1_f64 * t3904 + 0.65854491829355115987e0_f64 * t5346 + 0.26341796731742046395e1_f64 * t3906 - 0.13170898365871023197e1_f64 * t3910 - t3914 - 0.13170898365871023197e1_f64 * t5352 + 0.13170898365871023197e1_f64 * t5354 + t3917 - 0.26341796731742046394e1_f64 * t3920 - t3925;
    (t5340, t5351, t5357)
}
