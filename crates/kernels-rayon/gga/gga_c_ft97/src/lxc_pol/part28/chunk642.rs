//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 642/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk642(t25591: f64, t83: f64, t3238: f64, t452: f64, t5722: f64, t25593: f64, t1332: f64, t3103: f64, t488: f64, t23294: f64, t925: f64, t1909: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26230 = t83 * t25591;
    let t26234 = t452 * t3238 * t5722;
    let t26237 = t83 * t25593;
    let t26240 = t1332 * t3103;
    let t26242 = t452 * t488 * t26240;
    let t26245 = t23294 * t925;
    let t26246 = t1909 * t26245;
    (t26230, t26234, t26237, t26240, t26242, t26245, t26246)
}
