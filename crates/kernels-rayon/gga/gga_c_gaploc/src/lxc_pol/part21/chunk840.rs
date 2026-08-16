//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 840/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk840(t1445: f64, t8147: f64, t1265: f64, t2854: f64, t2765: f64, t524: f64, t188: f64, t7930: f64, t1457: f64, t7996: f64, t8012: f64, t7957: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8148 = t1445 * t8147;
    let t8151 = t2854 * t1265;
    let t8152 = t1445 * t8151;
    let t8155 = t524 * t2765;
    let t8158 = t188 * t7930;
    let t8165 = t1457 * t7996;
    let t8168 = t1457 * t8012;
    let t8171 = t1445 * t7957;
    (t8148, t8152, t8155, t8158, t8165, t8168, t8171)
}
