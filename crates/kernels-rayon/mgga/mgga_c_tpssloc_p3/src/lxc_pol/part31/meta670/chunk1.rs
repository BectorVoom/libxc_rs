//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1990/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1990(t100562: f64, t16662: f64, t16944: f64, t16949: f64, t17109: f64, t1877: f64, t2057: f64, t24344: f64, t2522: f64, t25365: f64, t25374: f64, t26563: f64, t26744: f64, t4119: f64, t4314: f64, t5527: f64, t5544: f64, t5664: f64, t67128: f64, t67164: f64, t7110: f64, t7114: f64, t7845: f64, t84800: f64, t93000: f64, t98007: f64, t98011: f64, t98030: f64) -> f64 {
    let t101892 = 3.0_f64 * t16662 * t2057 * t2522 + 12.0_f64 * t16944 * t2057 * t4314 + 6.0_f64 * t16949 * t2057 * t4314 - t17109 * t1877 * t7114 + 4.0_f64 * t1877 * t24344 * t98030 + 4.0_f64 * t1877 * t25374 * t93000 + 2.0_f64 * t1877 * t5664 * t84800 - 6.0_f64 * t2522 * t25365 * t26744 + 6.0_f64 * t2522 * t4119 * t7845 + 3.0_f64 * t2522 * t5544 * t7110 - 6.0_f64 * t2522 * t67164 * t7114 - 6.0_f64 * t2522 * t7114 * t98007 - 3.0_f64 * t2522 * t7114 * t98011 + 6.0_f64 * t4314 * t5527 * t7110 - 6.0_f64 * t4314 * t67128 * t7114 - 12.0_f64 * t100562 * t26563;
    t101892
}
