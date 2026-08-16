//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 782/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk782(t4167: f64, t883: f64, t900: f64, t1: f64, t30795: f64, t544: f64, t10525: f64, t2365: f64, t30136: f64, t12541: f64, t7014: f64, t2464: f64, t2465: f64, t2487: f64, t9193: f64) -> (f64, f64, f64, f64) {
    let t40165 = t883 * t4167;
    let t40166 = t900 * t40165;
    let t40167 = t544 * t30795 * t1 * t40166;
    let t40170 = t10525 * t2365 * t30136;
    let t40172 = t7014 * t12541;
    let t40176 = t2487 * t2464 * t2465 * t9193;
    (t40167, t40170, t40172, t40176)
}
