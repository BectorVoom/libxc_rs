//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 880/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk880(t571: f64, t8021: f64, t6240: f64, t928: f64, t259: f64, t6360: f64, t2572: f64, t7518: f64, t360: f64, t1570: f64, t1554: f64, t2124: f64, t2545: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8022 = t571 * t8021;
    let t8026 = t6240 * t928;
    let t8028 = t6360 * t259;
    let t8029 = t571 * t8028;
    let t8030 = t2572 * t7518;
    let t8031 = t360 * t8030;
    let t8034 = t2572 * t1570;
    let t8035 = t360 * t8034;
    let t8039 = t2124 * t2545 * t1554;
    (t8022, t8026, t8029, t8030, t8031, t8034, t8035, t8039)
}
