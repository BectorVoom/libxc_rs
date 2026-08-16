//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2514/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2514(t19572: f64, t4983: f64, t4998: f64, t19482: f64, t999: f64, t19501: f64, t1089: f64, t1678: f64, t4866: f64, t3153: f64, t6271: f64, t3298: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19573 = t19572 * t4983;
    let t19576 = t19572 * t4998;
    let t19579 = t19482 * t999;
    let t19580 = t19501 * t19579;
    let t19584 = t1678 * t4866 * t1089;
    let t19593 = t6271 * t3153;
    let t19594 = t19593 * t4983;
    let t19597 = t19593 * t4998;
    let t19602 = t3298 * t1678;
    (t19573, t19576, t19580, t19584, t19593, t19594, t19597, t19602)
}
