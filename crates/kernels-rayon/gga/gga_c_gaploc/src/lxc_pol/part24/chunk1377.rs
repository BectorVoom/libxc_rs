//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1377/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1377(t31585: f64, t4130: f64, t4781: f64, t590: f64, t31590: f64, t30572: f64, t18313: f64, t986: f64, t31119: f64, t6907: f64, t10600: f64, t1397: f64, t1424: f64) -> (f64, f64, f64, f64, f64) {
    let t34431 = 0.30674340763136599742e1_f64 * t4781 * t4130 * t31585 * t590;
    let t34435 = 0.30674340763136599742e1_f64 * t4781 * t4130 * t31590 * t590;
    let t34436 = 0.63904876589867916128e-1_f64 * t30572;
    let t34439 = t18313 * t986;
    let t34441 = t31119 * t34439 * t6907;
    let t34442 = 0.23005755572352449806e1_f64 * t34441;
    let t34445 = 0.79445533226334281486e-1_f64 * t1397 * t10600 * t1424;
    (t34431, t34435, t34436, t34442, t34445)
}
