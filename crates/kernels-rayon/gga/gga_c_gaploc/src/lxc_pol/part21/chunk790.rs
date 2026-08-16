//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 790/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk790(t1445: f64, t7487: f64, t5750: f64, t935: f64, t1865: f64, t7227: f64, t2581: f64, t4371: f64, t944: f64, t958: f64, t2668: f64, t4614: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7488 = t1445 * t7487;
    let t7491 = t5750 * t935;
    let t7492 = t7491 * t1865;
    let t7493 = t1445 * t7492;
    let t7496 = t1445 * t7227;
    let t7499 = t2581 * t1865;
    let t7500 = t1445 * t7499;
    let t7503 = t4371 * t944;
    let t7504 = t958 * t7503;
    let t7506 = t4614 * t2668;
    (t7488, t7493, t7496, t7499, t7500, t7503, t7504, t7506)
}
