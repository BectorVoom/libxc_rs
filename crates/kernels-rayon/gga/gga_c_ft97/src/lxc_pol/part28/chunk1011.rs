//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1011/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1011(t22940: f64, t6557: f64, t22914: f64, t34553: f64, t7211: f64, t984: f64, t34613: f64, t92: f64, t1286: f64, t34580: f64, t376: f64, t25542: f64, t7162: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t144704 = t22940 * t6557;
    let t144708 = t22914 * t34553;
    let t144714 = t7211 * t984;
    let t144719 = t34613 * t92;
    let t144725 = t1286 * t376 * t34580;
    let t144727 = t7162 * t25542;
    (t144704, t144708, t144714, t144719, t144725, t144727)
}
