//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1354/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1354(t10547: f64, t6820: f64, t204: f64, t2476: f64, t34411: f64, t34407: f64, t6710: f64, t6711: f64, t1429: f64, t2365: f64, t2366: f64, t7861: f64) -> (f64, f64, f64, f64) {
    let t35185 = 0.25025342966295298669e1_f64 * t10547 * t6820;
    let t35188 = 0.46011511144704899612e1_f64 * t2476 * t204 * t34411;
    let t35192 = 0.23005755572352449806e2_f64 * t6710 * t6711 * t34407;
    let t35198 = t1429 * t2365 * t2366 * t7861;
    (t35185, t35188, t35192, t35198)
}
