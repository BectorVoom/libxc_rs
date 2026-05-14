//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 552/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk552<F: Float>(t407: F, t5315: F, t1160: F, t1655: F, t310: F, t547: F, t848: F, t449: F, t556: F, t864: F, t863: F, t1614: F, t852: F, t3896: F, t557: F, t545: F, t851: F) -> (F, F, F, F, F, F, F, F) {
    let t5316 = t5315 * t407;
    let t5318 = 0.13170898365871023197e1 * t1160 * t5316;
    let t5327 = 0.13170898365871023197e1 * t310 * t1655;
    let t5346 = t848 * t547;
    let t5351 = t449 * t556 * t864;
    let t5352 = t863 * t5351;
    let t5354 = t852 * t1614;
    let t5359 = 0.13170898365871023197e1 * t3896 * t557;
    let t5360 = t851 * t545;
    (t5318, t5327, t5346, t5351, t5352, t5354, t5359, t5360)
}
