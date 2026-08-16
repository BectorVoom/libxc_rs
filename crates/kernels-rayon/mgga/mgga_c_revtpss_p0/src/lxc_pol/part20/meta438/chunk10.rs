//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1661/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1661(t300: f64, t45173: f64, t45218: f64, t45251: f64, t45293: f64, t12596: f64, t3531: f64, t1196: f64, t12552: f64, t3523: f64, t43753: f64, t1188: f64, t12485: f64) -> (f64, f64, f64, f64) {
    let t45296 = t300 * (t45173 + t45218 + t45251 + t45293);
    let t45298 = 0.14035736694323150897e2_f64 * t3531 * t12596;
    let t45302 = 0.6233709278045326953e3_f64 * t1196 * t12552 * t43753 * t3523;
    let t45306 = 0.14035736694323150897e2_f64 * t1196 * t12485 * t43753 * t1188;
    (t45296, t45298, t45302, t45306)
}
