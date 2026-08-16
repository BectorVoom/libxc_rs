//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 830/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk830(t147: f64, t32741: f64, t33229: f64, t184: f64, t5: f64, t7419: f64, t21: f64, t363: f64, t650: f64, t7420: f64, t1337: f64, t942: f64, t5507: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t148 = 10000000.0_f64 <= t147;
    let t33230 = t32741 + t33229;
    let t33231 = t33230 * t184;
    let t33234 = t5 * t7419;
    let t33240 = piecewise3(t148, 0.0_f64, t5 * t33231 * t21 / 4.0_f64 + t5 * t7420 * t363 / 4.0_f64 + t33234 * t650 / 4.0_f64);
    let t34352 = t1337 * t942;
    let t34353 = t5507 * t34352;
    (t33230, t33231, t33234, t33240, t34352, t34353)
}
