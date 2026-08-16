//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 761/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk761(t1908: f64, t198: f64, t4043: f64, t5059: f64, t8748: f64, t654: f64, t667: f64, t116: f64, t3028: f64, t3163: f64, t3153: f64, t3157: f64, t561: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8751 = t4043 * t198 * t1908 * t5059;
    let t8752 = t8748 * t8751;
    let t8754 = t654 * t667;
    let t8755 = t116 * t8754;
    let t8756 = t8755 * t8751;
    let t8758 = t3028 * t3163;
    let t8761 = t561 * t3153 * t3157;
    (t8751, t8752, t8754, t8756, t8758, t8761)
}
