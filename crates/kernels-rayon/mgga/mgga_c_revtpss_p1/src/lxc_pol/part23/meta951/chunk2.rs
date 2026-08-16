//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3151/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3151(t1214: f64, t24734: f64, t1248: f64, t12809: f64, t12855: f64, t17261: f64, t17459: f64, t17747: f64, t20800: f64, t21049: f64, t21121: f64, t21223: f64, t24715: f64, t24729: f64, t24739: f64, t3604: f64, t3720: f64, t44609: f64, t5284: f64, t56786: f64, t56791: f64, t59162: f64, t6688: f64, t69856: f64, t69866: f64, t70890: f64) -> (f64, f64) {
    let t82775 = t24734 * t1214;
    let t82792 = 0.12862205435420921092e-2_f64 * t17261 * t24715 + 0.47637797908966374413e-3_f64 * t69856 + t56786 + t56791 - 0.85748036236139473947e-3_f64 * t21049 * t21223 - 0.38586616306262763276e-2_f64 * t17747 * t3720 * t70890 * t24729 * t1248 + 0.64311027177104605458e-3_f64 * t12809 * t3720 * t20800 * t82775 - 0.25724410870841842184e-2_f64 * t59162 * t21121 - 0.25724410870841842184e-2_f64 * t12855 * t3720 * t6688 * t3604 * t5284 + 0.30488190661738479624e-2_f64 * t69866 - 0.38586616306262763276e-2_f64 * t44609 * t3720 * t24739 * t17459;
    (t82775, t82792)
}
