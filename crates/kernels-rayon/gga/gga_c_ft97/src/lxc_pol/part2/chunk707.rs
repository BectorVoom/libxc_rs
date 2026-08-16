//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 707/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk707(t11139: f64, t11141: f64, t1655: f64, t929: f64, t35: f64, t1594: f64, t11084: f64, t534: f64, t1595: f64, t1630: f64, t3064: f64, t3020: f64, t3070: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11142 = t11139 * t11141;
    let t11145 = t929 * t1655;
    let t11146 = t11145 * t35;
    let t11147 = t1594 * t11146;
    let t11150 = t534 * t11084;
    let t11153 = t1630 * t1595;
    let t11154 = t11153 * t35;
    let t11155 = t3064 * t11154;
    let t11160 = t3020 * t3070;
    (t11142, t11145, t11146, t11147, t11150, t11155, t11160)
}
