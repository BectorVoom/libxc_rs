//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 940/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk940(t15: f64, t17: f64, t218: f64, t33424: f64, t33428: f64, t24357: f64, t33380: f64, t173: f64, t33373: f64, t27521: f64, t7470: f64, t1418: f64, t33372: f64) -> (f64, f64, f64, f64, f64) {
    let t141058 = t218 * t15 * t17;
    let t141060 = t33424 * t141058 * t33428;
    let t141071 = t33380 * t24357;
    let t141073 = t173 * t33373;
    let t141075 = t27521 * t7470 * t141073;
    let t141082 = t33372 * t1418 * t141073;
    (t141058, t141060, t141071, t141075, t141082)
}
