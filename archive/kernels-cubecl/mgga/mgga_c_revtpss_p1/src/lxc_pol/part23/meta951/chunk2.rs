//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3151/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3151<F: Float>(t1214: F, t24734: F, t1248: F, t12809: F, t12855: F, t17261: F, t17459: F, t17747: F, t20800: F, t21049: F, t21121: F, t21223: F, t24715: F, t24729: F, t24739: F, t3604: F, t3720: F, t44609: F, t5284: F, t56786: F, t56791: F, t59162: F, t6688: F, t69856: F, t69866: F, t70890: F) -> (F, F) {
    let t82775 = t24734 * t1214;
    let t82792 = F::cast_from(0.12862205435420921092e-2_f64) * t17261 * t24715 + F::cast_from(0.47637797908966374413e-3_f64) * t69856 + t56786 + t56791 - F::cast_from(0.85748036236139473947e-3_f64) * t21049 * t21223 - F::cast_from(0.38586616306262763276e-2_f64) * t17747 * t3720 * t70890 * t24729 * t1248 + F::cast_from(0.64311027177104605458e-3_f64) * t12809 * t3720 * t20800 * t82775 - F::cast_from(0.25724410870841842184e-2_f64) * t59162 * t21121 - F::cast_from(0.25724410870841842184e-2_f64) * t12855 * t3720 * t6688 * t3604 * t5284 + F::cast_from(0.30488190661738479624e-2_f64) * t69866 - F::cast_from(0.38586616306262763276e-2_f64) * t44609 * t3720 * t24739 * t17459;
    (t82775, t82792)
}
