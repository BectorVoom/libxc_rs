//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 953/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk953(t13109: f64, t13113: f64, t16702: f64, t185: f64, t20234: f64, t9897: f64, t1462: f64, t16689: f64, t13124: f64, t16711: f64, t9853: f64, t9859: f64, t9907: f64, t9921: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20822 = 0.73245789224026180216e-3_f64 * t13109;
    let t20823 = 0.17544670867903938621e1_f64 * t13113;
    let t20824 = 12.0_f64 * t16702;
    let t20825 = t185 * t20234;
    let t20827 = 24.0_f64 * t9897 * t20825;
    let t20829 = 12.0_f64 * t16689 * t1462;
    let t20830 = 0.32530743900905219526e-1_f64 * t13124;
    let t20831 = 0.54934341918019635162e-3_f64 * t16711;
    let t20832 = t20822 + t9907 - t20823 + t20824 + t20827 + t9853 + t20829 - t9921 + t20830 - t20831 + t9859;
    (t20822, t20823, t20824, t20827, t20829, t20830, t20831, t20832)
}
