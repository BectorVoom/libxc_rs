//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1041/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1041(t2615: f64, t326: f64, t43586: f64, t13146: f64, t5676: f64, t13077: f64, t7712: f64, t3040: f64, t41468: f64, t2536: f64, t3431: f64, t2009: f64, t2021: f64) -> (f64, f64, f64, f64, f64) {
    let t43815 = t2615 * t326 * t43586;
    let t43817 = t5676 * t13146;
    let t43820 = 0.71500979903700853338e0_f64 * t13077 * t7712;
    let t43822 = 0.35750489951850426669e0_f64 * t41468 * t3040;
    let t43823 = t2536 * t3431;
    let t43825 = t2021 * t43823 * t2009;
    (t43815, t43817, t43820, t43822, t43825)
}
