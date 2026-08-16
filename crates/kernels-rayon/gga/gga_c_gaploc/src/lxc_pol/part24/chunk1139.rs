//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1139/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1139(t2464: f64, t2465: f64, t587: f64, t6417: f64, t2487: f64, t6428: f64, t3178: f64, t4625: f64, t1407: f64, t9279: f64, t9555: f64, t3193: f64, t4634: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30374 = 0.17041300423964777634e0_f64 * t587 * t2464 * t2465 * t6417;
    let t30378 = 0.17041300423964777634e0_f64 * t2487 * t2464 * t2465 * t6428;
    let t30379 = t4625 * t3178;
    let t30380 = 0.38342925953920749676e0_f64 * t30379;
    let t30381 = t1407 * t9279;
    let t30382 = 0.76685851907841499352e0_f64 * t30381;
    let t30387 = t1407 * t9555;
    let t30388 = 0.1022478025437886658e1_f64 * t30387;
    let t30404 = t4634 * t3193;
    (t30374, t30378, t30380, t30382, t30388, t30404)
}
