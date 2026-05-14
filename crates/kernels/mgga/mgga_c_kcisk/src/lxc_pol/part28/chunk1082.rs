//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1082/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1082<F: Float>(t11999: F, t12095: F, t18541: F, t22870: F, t22872: F, t22888: F, t22893: F, t22895: F, t22897: F, t22899: F, t22901: F, t2605: F, t5368: F, t7467: F, t7490: F, t9109: F, t9125: F, t9140: F) -> (F,) {
    let t24800 = 2.0 * t18541 * t2605 + 2.0 * t7467 * t7490 - 2.0 * t11999 * t9109 + 1.0 * t5368 * t9125 - t22870 - t22872 - t22893 - t22895 - t22897 + t22899 - t22901 + 0.17315755899375863299e2 * t12095 * t9140 - 0.19751789702565206229e-1 * t22888;
    (t24800,)
}
