//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 946/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk946(t32300: f64, t66: f64, t1616: f64, t37481: f64, t53: f64, t7189: f64, t1613: f64, t5555: f64, t409: f64, t5517: f64, t1301: f64, t136505: f64, t32259: f64) -> (f64, f64, f64, f64, f64) {
    let t136735 = t32300 * t66;
    let t136736 = t136735 * t1616;
    let t136740 = t37481 * t53 * t7189;
    let t136759 = t1613 * t5555;
    let t136772 = t5517 * t409;
    let t136807 = t32259 * t1301 * t136505;
    (t136736, t136740, t136759, t136772, t136807)
}
