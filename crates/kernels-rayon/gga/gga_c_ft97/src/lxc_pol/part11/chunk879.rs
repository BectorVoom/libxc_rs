//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 879/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk879(t37453: f64, t7906: f64, t1648: f64, t1771: f64, t458: f64, t7960: f64, t7963: f64, t7967: f64, t7970: f64, t17: f64, t37352: f64, t37355: f64, t37357: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t38037 = t7906 * t37453;
    let t38042 = t1771 * t1648;
    let t38044 = t458 * t7960;
    let t38046 = t458 * t7963;
    let t38048 = t458 * t7967;
    let t38050 = t458 * t7970;
    let t38052 = t17 * t37352;
    let t38053 = t37355 * t37357;
    (t38037, t38042, t38044, t38046, t38048, t38050, t38052, t38053)
}
