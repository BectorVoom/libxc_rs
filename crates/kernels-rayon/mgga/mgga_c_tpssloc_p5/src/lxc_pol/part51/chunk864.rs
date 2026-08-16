//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 864/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk864(t8466: f64, t8467: f64, t1998: f64, t2006: f64, t214: f64, t1985: f64, t1401: f64, t8326: f64, t63: f64, t8301: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8468 = t8466 * t8467;
    let t8479 = t1998 * t2006;
    let t8480 = t214 * t8479;
    let t8482 = 0.16449340668482264365e-1_f64 * t1985 * t8480;
    let t8508 = 0.135e2_f64 * t1401 * t8326;
    let t8511 = t8301 * t63;
    (t8468, t8479, t8480, t8482, t8508, t8511)
}
