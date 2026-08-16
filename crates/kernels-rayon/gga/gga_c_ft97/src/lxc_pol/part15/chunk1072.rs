//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1072/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1072(t39674: f64, t61462: f64, t62134: f64, t86608: f64, t86937: f64, t86942: f64, t86946: f64, t86950: f64, t86954: f64, t86958: f64, t86962: f64, t86966: f64, t86970: f64, t86975: f64, t86979: f64) -> f64 {
    let t87113 = -15.0_f64 / 16.0_f64 * t86608 + t86937 / 2.0_f64 + 16.0_f64 / 9.0_f64 * t61462 + t39674 - 36.0_f64 * t86942 + 8.0_f64 * t86946 - 80.0_f64 / 81.0_f64 * t86950 - 8.0_f64 * t86954 - t86958 / 3.0_f64 + 8.0_f64 * t86962 + 2.0_f64 * t86966 - 2.0_f64 / 3.0_f64 * t86970 + 16.0_f64 / 3.0_f64 * t62134 - 8.0_f64 * t86975 + 8.0_f64 / 3.0_f64 * t86979;
    t87113
}
