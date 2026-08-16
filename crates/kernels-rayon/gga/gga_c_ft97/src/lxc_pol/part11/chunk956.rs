//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 956/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk956(t137: f64, t8906: f64, t135: f64, t2059: f64, t2071: f64, t2030: f64, t2035: f64, t2037: f64, t1701: f64, t37614: f64, t538: f64, t554: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39801 = 1.0_f64 / t8906 / t137;
    let t39802 = t135 * t39801;
    let t39803 = t2059 * t2059;
    let t39807 = t2071 * t2071;
    let t39813 = t2030 * t2030;
    let t39818 = t2035 * t2037 * t2071;
    let t39824 = t1701 * t37614 * t538;
    let t39828 = t1701 * t37614 * t554;
    (t39802, t39803, t39807, t39813, t39818, t39824, t39828)
}
