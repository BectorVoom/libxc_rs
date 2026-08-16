//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 920/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk920(t2101: f64, t5935: f64, t5842: f64, t604: f64, t23571: f64, t50235: f64, t5617: f64, t984: f64, t25752: f64, t45499: f64, t35: f64, t358: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95789 = t2101 * t5935;
    let t95813 = t604 * t5842;
    let t95842 = t50235 * t23571;
    let t100089 = t5617 * t984;
    let t100483 = t45499 * t25752;
    let t100775 = t35 * t358;
    (t95789, t95813, t95842, t100089, t100483, t100775)
}
