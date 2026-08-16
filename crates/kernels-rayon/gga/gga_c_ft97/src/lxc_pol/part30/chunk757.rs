//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 757/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk757(t342: f64, t630: f64, t7430: f64, t231: f64, t6061: f64, t1403: f64, t1526: f64, t2: f64, t2320: f64, t33540: f64, t33545: f64, t33547: f64, t33552: f64, t343: f64, t6136: f64, t6141: f64, t7426: f64, t7427: f64) -> (f64, f64, f64) {
    let t33557 = t342 * t630 * t7430 / 12.0_f64;
    let t33561 = t231 * t6061;
    let t33566 = (-t33540 * t7427 / 6.0_f64 + t33545 + t1403 * t33547 / 18.0_f64 + t1403 * t6141 / 3.0_f64 - t7426 * t33552 / 6.0_f64 - t33557 - t1526 * t2320 * t6136 / 12.0_f64 - t342 * t343 * t33561 / 4.0_f64) * t2;
    (t33557, t33561, t33566)
}
