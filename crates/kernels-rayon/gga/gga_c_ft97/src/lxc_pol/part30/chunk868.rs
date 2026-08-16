//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 868/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk868(t2781: f64, t35863: f64, t1486: f64, t193: f64, t1208: f64, t230: f64, t420: f64, t7470: f64, t1196: f64, t287: f64, t35462: f64, t290: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35864 = t2781 * t35863;
    let t35866 = t1486 * t193 * t35864;
    let t35870 = t230 * t1208;
    let t35871 = t420 * t35870;
    let t35872 = t7470 * t35871;
    let t35877 = t230 * t1196;
    let t35878 = t420 * t35877;
    let t35879 = t7470 * t35878;
    let t35886 = t35462 * t287;
    let t35887 = t35886 * t290;
    (t35864, t35866, t35870, t35872, t35877, t35879, t35887)
}
