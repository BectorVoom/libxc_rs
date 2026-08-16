//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1046/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1046(t1112: f64, t8598: f64, t3483: f64, t8601: f64, t2964: f64, t3537: f64, t3808: f64, t4908: f64, t687: f64, t4915: f64, t1049: f64, t1616: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12046 = t8598 * t1112;
    let t12047 = t8601 * t3483;
    let t12048 = 2.0_f64 * t12047;
    let t12049 = t2964 * t3537;
    let t12050 = t4908 * t3808;
    let t12051 = 2.0_f64 * t12050;
    let t12052 = t3808 * t687;
    let t12053 = t4915 * t12052;
    let t12054 = 6.0_f64 * t12053;
    let t12055 = t3537 * t1049;
    let t12056 = t1616 * t12055;
    (t12046, t12047, t12048, t12049, t12050, t12051, t12052, t12053, t12054, t12055, t12056)
}
