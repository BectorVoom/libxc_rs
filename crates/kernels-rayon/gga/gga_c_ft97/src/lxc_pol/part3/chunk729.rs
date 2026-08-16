//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 729/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk729(t1200: f64, t14728: f64, t4088: f64, t816: f64, t287: f64, t4061: f64, t1471: f64, t800: f64, t13596: f64, t1213: f64, t1636: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14742 = t1200 * t14728;
    let t14752 = t816 * t4088;
    let t14763 = t4061 * t287;
    let t14766 = t800 * t1471;
    let t14788 = 0.22226000364197530866e-1_f64 * t13596;
    let t14895 = t89 * t1636 * t1213;
    (t14742, t14752, t14763, t14766, t14788, t14895)
}
