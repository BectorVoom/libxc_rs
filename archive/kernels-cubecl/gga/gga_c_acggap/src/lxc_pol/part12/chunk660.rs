//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 660/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk660<F: Float>(t1220: F, t1264: F, t556: F, t547: F, t848: F, t449: F, t864: F, t863: F, t1614: F, t852: F, t1222: F, t1608: F, t3902: F, t3904: F, t3906: F, t3910: F, t3914: F, t3917: F, t3920: F, t3925: F, t446: F) -> (F, F, F) {
    let t5340 = t1220 * t556 * t1264;
    let t5346 = t848 * t547;
    let t5351 = t449 * t556 * t864;
    let t5352 = t863 * t5351;
    let t5354 = t852 * t1614;
    let t5357 = -F::cast_from(0.26341796731742046395e1_f64) * t3902 + F::cast_from(0.13170898365871023197e1_f64) * t446 * t5340 + F::cast_from(0.13170898365871023197e1_f64) * t1608 * t1222 + F::cast_from(0.13170898365871023197e1_f64) * t3904 + F::cast_from(0.65854491829355115987e0_f64) * t5346 + F::cast_from(0.26341796731742046395e1_f64) * t3906 - F::cast_from(0.13170898365871023197e1_f64) * t3910 - t3914 - F::cast_from(0.13170898365871023197e1_f64) * t5352 + F::cast_from(0.13170898365871023197e1_f64) * t5354 + t3917 - F::cast_from(0.26341796731742046394e1_f64) * t3920 - t3925;
    (t5340, t5351, t5357)
}
