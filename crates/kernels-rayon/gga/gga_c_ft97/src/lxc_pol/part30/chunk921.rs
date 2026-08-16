//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 921/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk921(t28557: f64, t28676: f64, t213: f64, t668: f64, t22511: f64, t28658: f64, t7003: f64, t2691: f64, t4113: f64, t28719: f64, t317: f64, t2842: f64, t7091: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t111830 = t28676 * t28557;
    let t111831 = t213 * t668;
    let t111837 = t28658 * t22511;
    let t111838 = t7003 * t111837;
    let t112071 = t28676 * t111837;
    let t112156 = t2691 * t28557;
    let t112159 = t4113 * t111837;
    let t112268 = t2691 * t111837;
    let t112384 = t28719 * t317;
    let t112390 = t7091 * t2842;
    (t111830, t111831, t111838, t112071, t112156, t112159, t112268, t112384, t112390)
}
