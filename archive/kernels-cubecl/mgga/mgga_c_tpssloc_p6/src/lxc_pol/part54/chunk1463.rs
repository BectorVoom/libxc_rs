//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1463/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1463<F: Float>(t120120: F, t120122: F, t120124: F, t120130: F, t122736: F, t122737: F, t122738: F, t122739: F, t122740: F, t31236: F, t31238: F, t119824: F, t119826: F, t119830: F, t120887: F, t120888: F, t120891: F, t120892: F, t120896: F, t120899: F, t120900: F, t120907: F, t120910: F, t122721: F, t122723: F, t122724: F, t122725: F, t122726: F, t122727: F, t122730: F, t122731: F, t122734: F, t122735: F, t124867: F, t1266: F, t34137: F, t574: F) -> F {
    let t124870 = t122736 + t122737 + t122738 + t122739 + t122740 + t31236 + t31238 + t120120 + t120122 + t120124 + t120130;
    let t124876 = -t120887 + t120888 - t120891 - t120892 + (t124867 + F::cast_from(2.0_f64) * t122721 + F::cast_from(2.0_f64) * t122723 + F::cast_from(2.0_f64) * t122724 + F::cast_from(2.0_f64) * t122725 + F::cast_from(2.0_f64) * t122726 + F::cast_from(2.0_f64) * t122727 + F::cast_from(2.0_f64) * t122730 + F::cast_from(2.0_f64) * t122731 + F::cast_from(2.0_f64) * t122734 + F::cast_from(2.0_f64) * t122735 + F::cast_from(2.0_f64) * t124870) * t574 - t120896 - t120899 - t34137 * t1266 - t119824 - t119826 - t119830 - t120900 + t120907 - t120910;
    t124876
}
