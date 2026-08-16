//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 116/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk116(t307: f64, t311: f64, t135: f64, t230: f64, t265: f64, t267: f64, t273: f64) -> (f64, f64) {
    let t314 = 1.0_f64 + 0.65854491829355115987e0_f64 * t307 * t311;
    let t315 = f64::ln(t314);
    let t318 = t135 * t273 * t315 - t230 + t265 + t267;
    (t314, t318)
}
