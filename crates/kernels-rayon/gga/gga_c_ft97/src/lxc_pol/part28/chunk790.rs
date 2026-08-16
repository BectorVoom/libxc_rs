//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 790/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk790(t32729: f64, t609: f64, t1384: f64, t23478: f64, t2142: f64, t7407: f64, t23408: f64, t5778: f64, t28: f64, t1349: f64, t1362: f64, t32686: f64, t32692: f64, t32696: f64, t32701: f64, t32703: f64, t32708: f64, t32711: f64, t32714: f64, t32719: f64, t32724: f64, t32727: f64, t564: f64, t5766: f64, t5772: f64, t5775: f64, t5845: f64, t7309: f64, t7346: f64, t7412: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32730 = t32729 * t609;
    let t32732 = t23478 * t1384;
    let t32735 = t2142 * t7407;
    let t32737 = t5778 * t23408;
    let t32738 = t28 * t32737;
    let t32741 = t32686 * t1362 / 6.0_f64 + t5766 * t7346 / 3.0_f64 + t1349 * t32692 / 3.0_f64 + t1349 * t32696 / 3.0_f64 - t32701 - t32703 + t7309 * t5845 / 6.0_f64 - t32708 - t1349 * t32711 / 3.0_f64 - t32714 * t5775 / 18.0_f64 + t5772 * t32719 / 9.0_f64 - t5772 * t32724 / 18.0_f64 - 4.0_f64 * t32727 - 2.0_f64 * t32730 - 4.0_f64 * t32732 - t564 * t7412 - 2.0_f64 * t32735 - 2.0_f64 / 3.0_f64 * t1349 * t32738;
    (t32730, t32732, t32735, t32737, t32738, t32741)
}
