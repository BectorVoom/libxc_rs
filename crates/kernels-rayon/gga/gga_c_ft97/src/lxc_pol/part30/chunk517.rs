//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 517/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk517(t287: f64, t4061: f64, t1471: f64, t800: f64, t1095: f64, t230: f64, t1240: f64, t2842: f64) -> (f64, f64, f64, f64) {
    let t14763 = t4061 * t287;
    let t14766 = t800 * t1471;
    let t14842 = t230 * t1095;
    let t15128 = t1240 * t2842;
    (t14763, t14766, t14842, t15128)
}
