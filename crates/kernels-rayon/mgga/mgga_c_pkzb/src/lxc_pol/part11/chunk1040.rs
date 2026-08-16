//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1040/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1040(t11143: f64, t11159: f64, t11231: f64, t11236: f64, t11238: f64, t11316: f64, t11318: f64, t11321: f64, t11325: f64, t11329: f64, t11355: f64, t11363: f64, t11536: f64, t135: f64, t273: f64, t957: f64) -> f64 {
    let t11540 = t11536 * t135 * t273 * t957 + t11143 - t11159 - t11231 + t11236 + t11238 + t11316 + t11318 - t11321 - t11325 + t11329 - t11355 + t11363;
    t11540
}
