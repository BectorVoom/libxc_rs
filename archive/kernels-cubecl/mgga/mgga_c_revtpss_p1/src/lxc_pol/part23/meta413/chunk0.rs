//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1793/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1793<F: Float>(t18413: F, t2723: F, t10726: F, t2661: F, t231: F, t2662: F, t10703: F, t221: F, t5966: F, t2674: F, t125: F, t5977: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18414 = t18413 * t2723;
    let t18415 = t10726 * t18414;
    let t18416 = t2661 * t18415;
    let t18418 = t18413 * t231;
    let t18419 = t2662 * t18418;
    let t18420 = t2661 * t18419;
    let t18423 = t10703 * t221 * t5966;
    let t18424 = t2674 * t18423;
    let t18426 = t125 * t5977;
    (t18414, t18415, t18416, t18418, t18419, t18420, t18423, t18424, t18426)
}
