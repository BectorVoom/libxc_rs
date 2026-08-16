//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 892/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk892(t1359: f64, t574: f64, t6725: f64, t1060: f64, t2185: f64, t7312: f64, t1053: f64, t7339: f64, t605: f64, t33096: f64, t33101: f64, t34811: f64, t34815: f64, t34820: f64, t34825: f64, t34829: f64, t34833: f64, t34837: f64, t34841: f64, t34846: f64) -> (f64, f64, f64, f64, f64) {
    let t35118 = t574 * t6725 * t1359;
    let t35122 = t2185 * t1060 * t7312;
    let t35125 = t7339 * t1053;
    let t35127 = t574 * t605 * t35125;
    let t35138 = t34811 / 2.0_f64 + t33096 + 2.0_f64 / 9.0_f64 * t34815 + 4.0_f64 / 3.0_f64 * t34820 - 2.0_f64 / 3.0_f64 * t34825 - t34829 / 6.0_f64 - t33101 - t34833 / 9.0_f64 - t34837 + 2.0_f64 / 3.0_f64 * t34841 + t34846 / 12.0_f64;
    (t35118, t35122, t35125, t35127, t35138)
}
