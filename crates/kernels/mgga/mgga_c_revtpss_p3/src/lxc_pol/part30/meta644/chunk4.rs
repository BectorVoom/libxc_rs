//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2265/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2265<F: Float>(t101613: F, t101617: F, t101619: F, t101621: F, t101625: F, t101628: F, t101632: F, t101634: F, t101640: F, t101642: F, t101645: F, t101648: F, t1461: F, t18208: F, t18214: F, t1918: F, t2170: F, t27102: F, t29480: F, t4162: F, t5802: F, t7696: F, t8245: F) -> F {
    let t105775 = F::new(6.0) * t1461 * t29480 + F::new(12.0) * t18208 * t2170 + F::new(3.0) * t18214 * t2170 + F::new(3.0) * t1918 * t27102 + F::new(6.0) * t4162 * t8245 + F::new(12.0) * t5802 * t7696 + t101613 + t101617 + t101619 + t101621 + t101625 + t101628 + t101632 + t101634 + t101640 + t101642 + t101645 + t101648;
    t105775
}
