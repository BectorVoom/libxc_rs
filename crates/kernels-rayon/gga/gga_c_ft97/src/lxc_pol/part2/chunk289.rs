//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 289/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk289(t1137: f64, t1169: f64, t1173: f64, t1175: f64, t247: f64, t263: f64, t792: f64, t992: f64, t666: f64, t89: f64, t1095: f64, t801: f64) -> (f64, f64, f64, f64) {
    let t1178 = -t1137 * t263 - t1173 * t247 - 2.0_f64 * t1169 + 2.0_f64 * t1175;
    let t1186 = t792 * t992;
    let t1188 = t89 * t666 * t1186;
    let t1190 = t801 * t1095;
    (t1178, t1186, t1188, t1190)
}
