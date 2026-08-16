//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1463/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1463(t120120: f64, t120122: f64, t120124: f64, t120130: f64, t122736: f64, t122737: f64, t122738: f64, t122739: f64, t122740: f64, t31236: f64, t31238: f64, t119824: f64, t119826: f64, t119830: f64, t120887: f64, t120888: f64, t120891: f64, t120892: f64, t120896: f64, t120899: f64, t120900: f64, t120907: f64, t120910: f64, t122721: f64, t122723: f64, t122724: f64, t122725: f64, t122726: f64, t122727: f64, t122730: f64, t122731: f64, t122734: f64, t122735: f64, t124867: f64, t1266: f64, t34137: f64, t574: f64) -> f64 {
    let t124870 = t122736 + t122737 + t122738 + t122739 + t122740 + t31236 + t31238 + t120120 + t120122 + t120124 + t120130;
    let t124876 = -t120887 + t120888 - t120891 - t120892 + (t124867 + 2.0_f64 * t122721 + 2.0_f64 * t122723 + 2.0_f64 * t122724 + 2.0_f64 * t122725 + 2.0_f64 * t122726 + 2.0_f64 * t122727 + 2.0_f64 * t122730 + 2.0_f64 * t122731 + 2.0_f64 * t122734 + 2.0_f64 * t122735 + 2.0_f64 * t124870) * t574 - t120896 - t120899 - t34137 * t1266 - t119824 - t119826 - t119830 - t120900 + t120907 - t120910;
    t124876
}
