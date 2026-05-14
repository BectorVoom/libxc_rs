//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1207/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1207<F: Float>(t2568: F, t5147: F, t6187: F, t10052: F, t5064: F, t1882: F, t31157: F, t10085: F, t10157: F, t110796: F, t110803: F, t110805: F, t110817: F, t110826: F, t1456: F, t18206: F, t18486: F, t1901: F, t242: F, t2469: F, t31189: F, t31239: F, t446: F, t6154: F, t729: F, t97809: F, t97815: F) -> (F, F, F) {
    let t122642 = t2568 * t6187 * t5147;
    let t122647 = t10052 * t6187 * t5064;
    let t122655 = t1882 * t31157;
    let t122657 = -2.0 * t446 * t10157 * t1456 * t18206 + 2.0 / 3.0 * t446 * t729 * t2469 * t31239 + t446 * t729 * t6154 * t18486 / 3.0 + 2.0 / 3.0 * t446 * t242 * t122642 - 2.0 * t446 * t242 * t122647 + 2.0 / 9.0 * t1901 * t10085 * t31189 - t110796 + t110803 + t110805 + t97809 - t110817 - t110826 - 4.0 / 27.0 * t97815 - 2.0 / 9.0 * t122655;
    (t122642, t122647, t122657)
}
