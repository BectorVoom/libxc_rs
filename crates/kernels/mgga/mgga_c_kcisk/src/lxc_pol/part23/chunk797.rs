//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 797/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk797<F: Float>(t1620: F, t2748: F, t9489: F, t9493: F, t9495: F, t9499: F, t9501: F, t9503: F, t9505: F, t9507: F) -> (F, F) {
    let t9560 = t2748 * t1620;
    let t9571 = 0.9375e-1 * t9489 - 0.9375e-1 * t9493 - 0.25e0 * t9495 + 0.625e-1 * t9499 - 0.20234375e-1 * t9501 + 0.20234375e-1 * t9503 + 0.10791666666666666667e0 * t9505 - 0.26979166666666666667e-1 * t9507;
    (t9560, t9571)
}
