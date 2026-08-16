//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 377/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk377(t2434: f64, t304: f64, t305: f64, t1771: f64, t303: f64, t2344: f64, t295: f64) -> (f64, f64, f64, f64) {
    let t2730 = 0.11113000182098765433e-1_f64 * t2434;
    let t2755 = 1.0_f64 / t305 / t304;
    let t2761 = 4.0_f64 / 9.0_f64 * t1771 * t303;
    let t2766 = t2344 * t295;
    (t2730, t2755, t2761, t2766)
}
