//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1416/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1416(t43819: f64, t43727: f64, t43729: f64, t43734: f64, t43737: f64, t43740: f64, t43743: f64, t43746: f64, t43811: f64, t43816: f64, t43823: f64, t43828: f64) -> f64 {
    let t43942 = 0.96141975308641975307e-1_f64 * t43819;
    let t43949 = -0.27469135802469135803e-1_f64 * t43811 + 0.24722222222222222222e-1_f64 * t43727 - 0.74166666666666666668e-1_f64 * t43729 + 0.61805555555555555555e-1_f64 * t43734 - 0.38456790123456790123e-1_f64 * t43816 + t43942 - 0.22249999999999999999e0_f64 * t43737 - 0.18541666666666666666e-1_f64 * t43823 - 0.24722222222222222222e-1_f64 * t43740 + 0.33375e0_f64 * t43743 + 0.55625000000000000001e-1_f64 * t43828 + 0.74166666666666666668e-1_f64 * t43746;
    t43949
}
