//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 526/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk526(t2999: f64, t665: f64, t2382: f64, t688: f64, t2379: f64, t223: f64, t226: f64) -> (f64, f64, f64) {
    let t3704 = t2999 * t665;
    let t3722 = t688 * t2382;
    let t3723 = t2379 * t3722;
    let t3724 = t223 * t226;
    (t3704, t3723, t3724)
}
