//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1135/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1135(t35997: f64, t375: f64, t89: f64, t1486: f64, t35864: f64, t681: f64, t33820: f64, t33822: f64, t37041: f64, t4162: f64, t898: f64, t143042: f64, t143112: f64, t28501: f64) -> (f64, f64, f64, f64) {
    let t153402 = t89 * t375 * t35997;
    let t153405 = t1486 * t681 * t35864;
    let t153414 = t33820 * t898 * t37041 * t33822 * t4162;
    let t153418 = t33820 * t143112 * t143042 * t28501;
    (t153402, t153405, t153414, t153418)
}
