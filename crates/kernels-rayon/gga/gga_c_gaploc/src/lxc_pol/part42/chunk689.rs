//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 689/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk689(t11167: f64, t874: f64, t1445: f64, t574: f64, t11434: f64, t901: f64, t11430: f64, t13261: f64, t597: f64, t2366: f64, t3529: f64, t2365: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13371 = t11167 * t874;
    let t13372 = t1445 * t13371;
    let t13374 = 0.46011511144704899612e1_f64 * t574 * t13372;
    let t13378 = t11434 * t901;
    let t13379 = 0.14896037479937677779e-1_f64 * t13378;
    let t13380 = t11430 * t901;
    let t13381 = 0.14896037479937677779e-1_f64 * t13380;
    let t13383 = t1445 * t13261;
    let t13385 = 0.11502877786176224903e2_f64 * t597 * t13383;
    let t13386 = t2366 * t3529;
    let t13387 = t2365 * t13386;
    (t13371, t13372, t13374, t13379, t13381, t13383, t13385, t13386, t13387)
}
