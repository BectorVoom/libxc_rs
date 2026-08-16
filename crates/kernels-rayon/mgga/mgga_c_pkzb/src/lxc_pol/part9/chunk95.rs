//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 95/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk95(t210: f64, t213: f64, t216: f64, t222: f64) -> (f64, f64, f64) {
    let t244 = 0.705945e1_f64 * t213 + 0.1549425e1_f64 * t210 + 0.420775e0_f64 * t216 + 0.1562925e0_f64 * t222;
    let t247 = 1.0_f64 + 0.32163958997385070134e2_f64 / t244;
    let t248 = f64::ln(t247);
    (t244, t247, t248)
}
