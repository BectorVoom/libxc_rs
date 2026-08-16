//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 950/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk950(t1403: f64, t2399: f64, t7486: f64, t7442: f64, t33568: f64, t5999: f64, t140768: f64, t141200: f64, t141203: f64, t141363: f64, t141367: f64, t24211: f64, t7437: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t141543 = 2.0_f64 / 27.0_f64 * t1403 * t2399 * t7486;
    let t141552 = 4.0_f64 / 27.0_f64 * t1403 * t2399 * t7442;
    let t141560 = t33568 * t5999;
    let t141577 = 2.0_f64 / 27.0_f64 * t140768;
    let t141606 = 8.0_f64 / 27.0_f64 * t141200;
    let t141607 = 4.0_f64 / 27.0_f64 * t141203;
    let t141651 = 4.0_f64 / 27.0_f64 * t141363;
    let t141652 = 10.0_f64 / 27.0_f64 * t141367;
    let t141671 = 2.0_f64 / 27.0_f64 * t7437 * t24211;
    (t141543, t141552, t141560, t141577, t141606, t141607, t141651, t141652, t141671)
}
