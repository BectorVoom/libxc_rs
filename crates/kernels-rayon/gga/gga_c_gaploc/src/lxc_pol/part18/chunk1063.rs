//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1063/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1063(t20157: f64, t2085: f64, t805: f64, t5654: f64, t7426: f64, t2032: f64, t6134: f64, t7177: f64, t900: f64, t10007: f64, t7068: f64, t10012: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22826 = t805 * t2085 * t20157;
    let t22854 = t5654 * t7426;
    let t22883 = t6134 * t2032;
    let t22909 = t900 * t7177;
    let t22980 = t10007 * t7068;
    let t22984 = t10012 * t7068;
    (t22826, t22854, t22883, t22909, t22980, t22984)
}
