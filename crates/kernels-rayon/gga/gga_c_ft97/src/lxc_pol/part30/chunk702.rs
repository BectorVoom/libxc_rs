//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 702/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk702(t24873: f64, t4260: f64, t15312: f64, t1501: f64, t2360: f64, t3886: f64, t15254: f64, t2347: f64, t15294: f64, t15299: f64, t28760: f64, t684: f64, t7045: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29198 = t24873 * t4260;
    let t29199 = t15312 * t29198;
    let t29202 = t1501 * t2360;
    let t29203 = t29202 * t3886;
    let t29204 = t15254 * t29203;
    let t29207 = t1501 * t2347;
    let t29208 = t29207 * t3886;
    let t29209 = t15294 * t29208;
    let t29212 = t15299 * t28760;
    let t29215 = t7045 * t684;
    (t29198, t29199, t29203, t29204, t29208, t29209, t29212, t29215)
}
