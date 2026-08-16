//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 932/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk932(t1403: f64, t33268: f64, t681: f64, t33531: f64, t761: f64, t33254: f64, t33574: f64, t33590: f64, t5996: f64, t2: f64, t33452: f64, t231: f64, t33301: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t140684 = t1403 * t681 * t33268;
    let t140686 = t33531 * t761;
    let t140707 = t1403 * t681 * t33254;
    let t140710 = t1403 * t681 * t33574;
    let t140712 = t5996 * t33590;
    let t140714 = t2 * t33452;
    let t140744 = t231 * t33301;
    (t140684, t140686, t140707, t140710, t140712, t140714, t140744)
}
