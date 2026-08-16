//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1063/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1063(t2616: f64, t486: f64, t1890: f64, t21783: f64, t20157: f64, t2085: f64, t805: f64, t5654: f64, t7426: f64, t2032: f64, t6134: f64, t7177: f64, t900: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22767 = t486 * t2616;
    let t22775 = t1890 * t21783;
    let t22826 = t805 * t2085 * t20157;
    let t22854 = t5654 * t7426;
    let t22883 = t6134 * t2032;
    let t22909 = t900 * t7177;
    (t22767, t22775, t22826, t22854, t22883, t22909)
}
