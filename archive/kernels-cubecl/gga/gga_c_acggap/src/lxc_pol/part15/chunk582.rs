//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 582/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk582<F: Float>(t381: F, t5304: F, t322: F, t545: F, t407: F, t1160: F, t1655: F, t310: F, t547: F, t848: F, t449: F, t556: F, t864: F) -> (F, F, F, F, F) {
    let t5305 = t381 * t5304;
    let t5315 = t545 * t322;
    let t5316 = t5315 * t407;
    let t5318 = F::cast_from(0.13170898365871023197e1_f64) * t1160 * t5316;
    let t5327 = F::cast_from(0.13170898365871023197e1_f64) * t310 * t1655;
    let t5346 = t848 * t547;
    let t5351 = t449 * t556 * t864;
    (t5305, t5318, t5327, t5346, t5351)
}
