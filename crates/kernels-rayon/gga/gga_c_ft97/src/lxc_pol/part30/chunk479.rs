//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 479/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk479(t317: f64, t7612: f64, t193: f64, t1477: f64, t1506: f64, t2862: f64, t319: f64, t7584: f64, t1476: f64, t1508: f64, t840: f64, t1501: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7613 = t7612 * t317;
    let t7614 = t193 * t7613;
    let t7617 = t1477 * t1506;
    let t7618 = t193 * t7617;
    let t7622 = t2862 * t319 * t7584;
    let t7626 = t840 * t1508 * t1476;
    let t7629 = t1476 * t1501;
    (t7613, t7614, t7617, t7618, t7622, t7626, t7629)
}
