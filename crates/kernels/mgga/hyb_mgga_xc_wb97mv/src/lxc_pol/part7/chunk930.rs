//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 930/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk930<F: Float>(t2039: F, t3188: F, t8651: F, t2065: F, t3: F, t674: F, t2013: F, t3189: F, t3194: F, t2033: F, t3195: F, t39: F, t697: F, t700: F, t3040: F, t35: F, t571: F, t6129: F, t6515: F, t6516: F, t6518: F, t6520: F, t6522: F, t8637: F, t8640: F, t8642: F, t8643: F, t8648: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8653 = t3188 * t8651 * t2039;
    let t8656 = t2065 * t3;
    let t8658 = t3188 * t8656 * t674;
    let t8662 = t3188 * t3189 * t2013;
    let t8666 = t3194 * t3189 * t2039;
    let t8669 = t2033 * t3;
    let t8671 = t3194 * t8669 * t674;
    let t8675 = t3194 * t3195 * t2013;
    let t8678 = t697 * t39;
    let t8679 = t8678 * t700;
    let t8683 = -t6515 - 4.0 / 243.0 * t6516 + t6518 / 243.0 - t6520 / 81.0 + t6522 / 162.0 - 2.0 / 243.0 * t8637 + t8640 - t8642 - 11.0 / 81.0 * t8643 - 5.0 / 243.0 * t571 * t8648 + 2.0 / 27.0 * t571 * t8653 + 4.0 / 81.0 * t3040 * t8658 - t571 * t8662 / 81.0 - t571 * t8666 / 9.0 - 4.0 / 27.0 * t3040 * t8671 + t571 * t8675 / 27.0 + t35 * t6129 * t8679 / 27.0;
    (t8653, t8658, t8662, t8666, t8671, t8675, t8678, t8679, t8683)
}
