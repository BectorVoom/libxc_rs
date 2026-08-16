//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1324/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1324(t3225: f64, t35834: f64, t10153: f64, t35751: f64, t6182: f64, t11683: f64, t11687: f64, t22442: f64, t11698: f64, t6178: f64, t297: f64, t825: f64) -> (f64, f64, f64, f64, f64) {
    let t35835 = t3225 * t35834;
    let t35838 = t10153 * t35751 * t6182;
    let t35841 = t11687 * t11683 * t22442;
    let t35843 = t6178 * t11698;
    let t35846 = t825 * t297;
    (t35835, t35838, t35841, t35843, t35846)
}
