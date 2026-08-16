//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1354/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1354(t19577: f64, t22574: f64, t36740: f64, t120874: f64, t120876: f64, t120877: f64, t120878: f64, t120881: f64, t120885: f64, t120887: f64, t120888: f64, t120891: f64, t120892: f64, t12725: f64, t5361: f64, t8529: f64, t8604: f64) -> f64 {
    let t120896 = 3.0_f64 * t22574 * t36740 * t19577;
    let t120897 = -2.0_f64 * t12725 * t8529 + t5361 * t8604 + t120874 + t120876 - t120877 - t120878 - t120881 + t120885 - t120887 + t120888 - t120891 - t120892 - t120896;
    t120897
}
