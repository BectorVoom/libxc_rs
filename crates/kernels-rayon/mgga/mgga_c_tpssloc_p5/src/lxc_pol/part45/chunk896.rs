//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 896/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk896(t1375: f64, t2016: f64, t2092: f64, t22670: f64, t24095: f64, t31094: f64, t31103: f64, t31129: f64, t31140: f64, t31552: f64, t31555: f64, t31561: f64, t31564: f64, t31571: f64, t31573: f64, t31597: f64, t31601: f64, t31609: f64, t31613: f64, t31642: f64, t31666: f64, t568: f64, t6958: f64, t6963: f64, t6993: f64, t7194: f64, t7199: f64, t7214: f64) -> f64 {
    let t31668 = 0.16449340668482264365e-1_f64 * t31552 + t31094 + 2.0_f64 * t1375 * t31555 + 0.16449340668482264365e-1_f64 * t31561 + 2.0_f64 * t1375 * t31564 + 2.0_f64 * t6958 * t7199 - t31571 + t31103 - t24095 * t2016 + t31573 * t568 + t31597 + t31129 - t6958 * t7214 + 2.0_f64 * t1375 * t31601 + 2.0_f64 * t7194 * t6963 - t7194 * t6993 - 0.82246703342411321825e-2_f64 * t31609 - 0.82246703342411321825e-2_f64 * t31613 - t22670 * t2092 - t1375 * t31642 - t31140 + t31666;
    t31668
}
