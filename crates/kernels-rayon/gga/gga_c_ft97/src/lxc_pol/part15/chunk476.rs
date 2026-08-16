//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 476/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk476(t4883: f64, t637: f64, t639: f64, t2251: f64, t2265: f64, t3611: f64, t3633: f64, t4857: f64, t4861: f64, t4865: f64, t4869: f64, t4874: f64, t631: f64) -> (f64, f64) {
    let t4885 = t637 * t639 * t4883;
    let t4888 = -t2251 - 2.0_f64 / 9.0_f64 * t3611 - 2.0_f64 / 3.0_f64 * t3633 + t631 * t4857 / 18.0_f64 - 2.0_f64 / 3.0_f64 * t2265 * t4861 - t631 * t4865 / 3.0_f64 + t631 * t4869 / 6.0_f64 - 3.0_f64 / 2.0_f64 * t631 * t4874 + t631 * t4885 / 2.0_f64;
    (t4885, t4888)
}
