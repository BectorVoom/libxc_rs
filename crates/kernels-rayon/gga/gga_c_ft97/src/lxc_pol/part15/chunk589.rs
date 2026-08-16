//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 589/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk589(t9680: f64, t226: f64, t10: f64, t242: f64, t3050: f64, t191: f64, t7514: f64, t27: f64, t9567: f64, t241: f64, t9570: f64, t2344: f64, t375: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9681 = 1.0_f64 / t9680;
    let t9682 = t226 * t9681;
    let t9698 = t10 * t3050 * t242;
    let t9699 = 14.0_f64 / 81.0_f64 * t9698;
    let t9707 = t191 * t7514;
    let t9716 = t27 * t9567;
    let t9717 = t241 * t9570;
    let t9725 = t375 * t2344;
    (t9681, t9682, t9698, t9699, t9707, t9716, t9717, t9725)
}
