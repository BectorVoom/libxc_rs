//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1147/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1147(t39706: f64, t39749: f64, t39803: f64, t39840: f64, t17: f64, t521: f64, t2225: f64, t3826: f64, t193: f64, t23857: f64, t3701: f64, t3914: f64, t39629: f64, t39631: f64, t39633: f64, t39635: f64, t39637: f64, t39640: f64, t39643: f64, t39645: f64, t39649: f64, t39655: f64, t39658: f64, t39660: f64, t5160: f64, t533: f64) -> (f64, f64, f64, f64) {
    let t39842 = t39706 + t39749 + t39803 + t39840;
    let t39844 = t17 * t521 * t39842;
    let t39845 = t2225 * t3826;
    let t39846 = 240.0_f64 * t39845;
    let t39847 = -3.0_f64 * t193 * t3701 * t39649 * t533 + 12.0_f64 * t23857 * t3914 * t5160 + t39629 + t39631 - t39633 + t39635 + t39637 + t39640 + t39643 - t39645 - t39655 - t39658 - t39660 + t39844 + t39846;
    (t39842, t39844, t39846, t39847)
}
