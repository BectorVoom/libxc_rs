//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1052/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1052<F: Float>(t42683: F, t102: F, t120: F, t48562: F, t10: F, t48737: F, t5825: F, t48741: F, t506: F, t128: F, t127: F, t12929: F, t25857: F, t25866: F, t3637: F, t3665: F, t42678: F, t42681: F, t42714: F, t496: F, t978: F) -> (F, F, F, F, F, F) {
    let t48777 = 0.116921e2 * t42683;
    let t48780 = 0.2923025e1 * t102 * t120 * t48562;
    let t48787 = t10 * t5825 * t48737;
    let t48791 = t10 * t506 * t48741;
    let t48795 = t10 * t128 * t48562;
    let t48807 = 0.587616e2 * t42678 + 2.0 / 3.0 * t42681 - t48777 - t48780 - 0.146904e1 * t127 * t506 * t48562 + 0.91406933333333333333e1 * t25857 + 0.783488e1 * t25866 + 30.0 * t496 * t48787 + 9.0 / 2.0 * t496 * t48791 - t496 * t48795 / 2.0 - 6.0 * t42714 - 36.0 * t496 * t10 * t3665 * t3637 + 6.0 * t496 * t10 * t978 * t12929;
    (t48777, t48780, t48787, t48791, t48795, t48807)
}
