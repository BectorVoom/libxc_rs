//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 754/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk754(t3066: f64, t32296: f64, t37: f64, t7171: f64, t78: f64, t371: f64, t5544: f64, t397: f64, t7203: f64, t7206: f64, t356: f64, t7204: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32297 = t32296 * t3066;
    let t32300 = t37 * t7171;
    let t32301 = t32300 * t78;
    let t32304 = t371 * t5544;
    let t32307 = t7203 * t397;
    let t32308 = t32307 * t7206;
    let t32311 = t7204 * t356;
    (t32297, t32300, t32301, t32304, t32307, t32308, t32311)
}
