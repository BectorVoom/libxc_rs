//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1105/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1105(t32174: f64, t34872: f64, t173: f64, t34871: f64, t7195: f64, t23839: f64, t26643: f64, t32233: f64, t3379: f64, t420: f64, t71: f64, t145074: f64, t23711: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t147274 = t32174 * t34872;
    let t147278 = t7195 * t173 * t34871;
    let t147279 = t23839 * t147278;
    let t147291 = t32233 * t26643;
    let t147298 = t7195 * t420 * t71 * t3379;
    let t147308 = t23711 * t145074;
    (t147274, t147278, t147279, t147291, t147298, t147308)
}
