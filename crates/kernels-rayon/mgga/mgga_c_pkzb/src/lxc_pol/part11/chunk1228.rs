//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1228/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1228(t3622: f64, t7560: f64, t20637: f64, t2852: f64, t30231: f64, t3626: f64, t2875: f64, t9242: f64, t10767: f64, t204: f64, t648: f64) -> (f64, f64, f64, f64, f64) {
    let t30270 = 0.17544670867903938621e1_f64 * t7560 * t3622;
    let t30273 = 0.31168546390226634766e3_f64 * t20637 * t2852 * t30231;
    let t30275 = 0.51947577317044391276e2_f64 * t7560 * t3626;
    let t30277 = 0.51947577317044391276e2_f64 * t9242 * t2875;
    let t30284 = t204 * t648 * t10767;
    (t30270, t30273, t30275, t30277, t30284)
}
