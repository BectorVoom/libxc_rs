//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 655/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk655<F: Float>(t3572: F, t3573: F, t5668: F, t5673: F, t5678: F, t5682: F) -> (F,) {
    let t5684 = t3572 + t3573 / 9.0 + t5668 / 9.0 - 2.0 / 9.0 * t5673 + 2.0 / 3.0 * t5678 - 2.0 / 3.0 * t5682;
    (t5684,)
}
