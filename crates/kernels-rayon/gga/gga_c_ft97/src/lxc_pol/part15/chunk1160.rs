//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1160/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1160(t66903: f64, t66906: f64, t66935: f64, t66946: f64, t67421: f64, t68751: f64, t68774: f64, t80685: f64, t80696: f64, t80759: f64, t88186: f64, t88190: f64, t88198: f64, t88201: f64) -> f64 {
    let t89712 = 4.0_f64 / 3.0_f64 * t88186 + 2.0_f64 / 9.0_f64 * t88190 + 4.0_f64 / 3.0_f64 * t80685 - t66903 + t66906 + 4.0_f64 / 9.0_f64 * t88198 - 4.0_f64 / 3.0_f64 * t88201 + 4.0_f64 / 9.0_f64 * t80696 + t66935 - t66946 + t68751 + t68774 - 8.0_f64 / 27.0_f64 * t80759 - t67421;
    t89712
}
