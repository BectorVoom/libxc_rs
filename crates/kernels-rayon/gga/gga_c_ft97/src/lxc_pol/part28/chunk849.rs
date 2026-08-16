//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 849/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk849(t32011: f64, t925: f64, t1564: f64, t32019: f64, t7824: f64, t22943: f64, t6547: f64, t7274: f64, t979: f64, t8418: f64, t1332: f64, t6557: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34552 = t32011 * t925;
    let t34553 = t1564 * t34552;
    let t34557 = t7824 * t32019 * t925;
    let t34560 = t22943 * t6547;
    let t34562 = t7274 * t979;
    let t34563 = t8418 * t34562;
    let t34565 = t1332 * t6557;
    (t34553, t34557, t34560, t34562, t34563, t34565)
}
