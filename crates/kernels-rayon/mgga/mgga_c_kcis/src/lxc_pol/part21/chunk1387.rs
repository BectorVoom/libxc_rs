//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1387/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1387(t27759: f64, t27761: f64, t28302: f64, t27153: f64, t27154: f64, t27157: f64, t27159: f64, t8: f64, t93848: f64, t93849: f64, t93852: f64, t97547: f64, t97567: f64, t97585: f64, t97602: f64) -> f64 {
    let t97606 = t27759 / 8.0_f64;
    let t97607 = t27761 / 8.0_f64;
    let t97608 = t28302 / 8.0_f64;
    let t97609 = t8 * (t97547 + t97567 + t97585 + t97602) - t97606 + t97607 - t27153 + t27154 + t93848 - t93849 - t27157 - t97608 + t27159 + t93852;
    t97609
}
