//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 815/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk815(t3424: f64, t363: f64, t12791: f64, t1557: f64, t586: f64, t10998: f64, t3506: f64, t11003: f64, t12561: f64, t24: f64, t1037: f64, t1771: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12792 = t3424 * t363;
    let t12793 = t12791 * t12792;
    let t12796 = t586 * t1557;
    let t12797 = t12796 * t12792;
    let t12800 = t3506 * t10998;
    let t12803 = t3506 * t11003;
    let t12807 = t24 * t586 * t12561;
    let t12809 = t1771 * t1037;
    (t12793, t12797, t12800, t12803, t12807, t12809)
}
