//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta314 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1341;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1342;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta314(t10470: f64, t11045: f64, t10471: f64, t10474: f64, t10482: f64, t6739: f64, t3127: f64, t3131: f64, t3215: f64, t390: f64, t268: f64, t405: f64, t6546: f64, t1091: f64, t2394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11046, t11059, t11060, t11065, t11066, t11094, t11135) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1341(t10470, t11045, t10471, t10474, t10482, t6739, t3127, t3131, t3215, t390, t268, t405, t6546);
        let (t11136, t11137) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1342(t11135, t1091, t2394);
    (t11046, t11059, t11060, t11065, t11066, t11094, t11135, t11136, t11137)
}
