//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 774/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk774(t2575: f64, t51: f64, t1727: f64, t2642: f64, t2607: f64, t501: f64, t2605: f64, t496: f64, t5086: f64, t5143: f64, t2557: f64, t545: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6999 = t51 * t2575;
    let t7009 = 0.20007875121765877254e-2_f64 * t1727 * t2642;
    let t7012 = t501 * t2607;
    let t7015 = 8.0_f64 * t496 * t2605;
    let t7017 = 8.0_f64 * t501 * t2605;
    let t7019 = 32.0_f64 * t5086;
    let t7022 = 48.0_f64 * t5143;
    let t7028 = t2557 * t545;
    (t6999, t7009, t7012, t7015, t7017, t7019, t7022, t7028)
}
