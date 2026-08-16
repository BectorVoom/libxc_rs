//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1407/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1407(t43727: f64, t43729: f64, t43734: f64, t43737: f64, t43740: f64, t43743: f64, t43746: f64, t43811: f64, t43816: f64, t43820: f64, t43823: f64, t43828: f64) -> f64 {
    let t43831 = -80.0_f64 / 81.0_f64 * t43811 + 8.0_f64 / 9.0_f64 * t43727 - 8.0_f64 / 3.0_f64 * t43729 + 20.0_f64 / 9.0_f64 * t43734 - 112.0_f64 / 81.0_f64 * t43816 + t43820 - 8.0_f64 * t43737 - 2.0_f64 / 3.0_f64 * t43823 - 8.0_f64 / 9.0_f64 * t43740 + 12.0_f64 * t43743 + 2.0_f64 * t43828 + 8.0_f64 / 3.0_f64 * t43746;
    t43831
}
