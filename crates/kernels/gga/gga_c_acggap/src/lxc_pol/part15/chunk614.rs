//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 614/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk614<F: Float>(t1215: F, t1608: F, t1620: F, t1938: F, t3906: F, t3914: F, t3917: F, t3920: F, t3925: F, t5346: F, t5352: F, t5354: F, t5359: F, t5361: F, t5364: F, t159: F, t1907: F) -> (F, F) {
    let t6453 = 0.26341796731742046394e1 * t1608 * t1620 + 0.13170898365871023197e1 * t5346 + 0.13170898365871023197e1 * t3906 - t3914 - 0.26341796731742046394e1 * t5352 + 0.26341796731742046394e1 * t5354 + t3917 - 0.13170898365871023197e1 * t3920 - t3925 - 0.65854491829355115987e0 * t1215 * t1938 - t5359 - 0.26341796731742046394e1 * t5361 + t5364;
    let t6454 = t159 * t1907;
    (t6453, t6454)
}
