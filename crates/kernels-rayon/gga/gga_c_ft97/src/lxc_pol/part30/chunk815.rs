//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 815/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk815(t33953: f64, t798: f64, t317: f64, t193: f64, t681: f64, t7613: f64, t1466: f64, t6213: f64, t7581: f64, t7585: f64, t880: f64, t7586: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34260 = t798 * t33953;
    let t34261 = t34260 * t317;
    let t34262 = t193 * t34261;
    let t34265 = t681 * t7613;
    let t34267 = t1466 * t34265 / 18.0_f64;
    let t34276 = t7581 * t6213 / 18.0_f64;
    let t34277 = t7585 * t880;
    let t34278 = t193 * t34277;
    let t34281 = t681 * t7586;
    (t34260, t34261, t34262, t34265, t34267, t34276, t34277, t34278, t34281)
}
