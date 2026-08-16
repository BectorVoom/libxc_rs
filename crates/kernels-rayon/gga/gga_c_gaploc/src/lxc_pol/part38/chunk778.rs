//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 778/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk778(t6125: f64, t883: f64, t286: f64, t39622: f64, t708: f64, t12557: f64, t2518: f64, t135: f64, t1691: f64, t458: f64, t5337: f64, t9105: f64) -> (f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t40594 = t883 * t6125;
    let t40612 = t39622 * t286 * t708;
    let t40614 = t2518 * t12557;
    let t40620 = t9105 * t5337 * pi * t1691 * t135 * t458;
    (t40594, t40612, t40614, t40620)
}
