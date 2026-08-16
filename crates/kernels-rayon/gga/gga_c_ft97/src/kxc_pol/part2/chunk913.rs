//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 913/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk913(t3871: f64, t8392: f64, t255: f64, t676: f64, t1168: f64, t2567: f64, t2579: f64, t1131: f64, t2373: f64, t10157: f64, t265: f64, t12001: f64, t3852: f64) -> (f64, f64, f64, f64) {
    let t14126 = 2.0_f64 / 27.0_f64 * t8392 * t3871;
    let t14127 = t676 * t255;
    let t14128 = t2567 * t1168;
    let t14129 = t14128 * t2579;
    let t14130 = t14127 * t14129;
    let t14133 = t1131 * t2373;
    let t14135 = t10157 * t265 * t14133;
    let t14138 = t12001 * t3852;
    (t14126, t14130, t14135, t14138)
}
