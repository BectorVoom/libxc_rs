//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 753/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk753(t1424: f64, t1454: f64, t2354: f64, t684: f64, t33290: f64, t33317: f64, t33286: f64, t33297: f64, t33305: f64, t33310: f64, t33314: f64, t33322: f64, t33326: f64, t33330: f64, t33335: f64) -> (f64, f64, f64, f64, f64) {
    let t33502 = t1424 * t1454;
    let t33504 = t2354 * t33502 * t684;
    let t33508 = 2.0_f64 / 9.0_f64 * t33290;
    let t33513 = t33317 / 9.0_f64;
    let t33517 = t33286 / 2.0_f64 + t33508 + 2.0_f64 / 9.0_f64 * t33297 + 4.0_f64 / 3.0_f64 * t33305 - 2.0_f64 / 3.0_f64 * t33310 - t33314 / 6.0_f64 - t33513 - t33322 / 9.0_f64 - t33326 + 2.0_f64 / 3.0_f64 * t33330 + t33335 / 12.0_f64;
    (t33502, t33504, t33508, t33513, t33517)
}
