//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 436/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk436<F: Float>(t2791: F, t2804: F, t2807: F, t2795: F, t2797: F) -> (F, F) {
    let t2811 = -0.52083333333333333333e-2 * t2804 * t2807 + 0.17411041666666666666e-2 * t2791;
    let t2815 = 0.9375e-1 * t2795 - 0.20234375e-1 * t2797;
    (t2811, t2815)
}
