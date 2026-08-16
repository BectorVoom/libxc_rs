//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1017/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1017(t23054: f64, t34412: f64, t34415: f64, t379: f64, t22958: f64, t5674: f64, t3204: f64, t32333: f64, t93355: f64, t144801: f64, t144792: f64, t137082: f64, t3188: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t144807 = t23054 * t34412;
    let t144809 = t34415 * t379;
    let t144811 = t5674 * t22958 * t144809;
    let t144813 = t32333 * t3204;
    let t144815 = t5674 * t93355 * t144813;
    let t144817 = t5674 * t22958 * t144801;
    let t144820 = t5674 * t22958 * t144792;
    let t144822 = t137082 * t3188;
    (t144807, t144809, t144811, t144813, t144815, t144817, t144820, t144822)
}
