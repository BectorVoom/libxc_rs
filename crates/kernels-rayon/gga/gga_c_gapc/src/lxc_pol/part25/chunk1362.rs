//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1362/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1362(t3537: f64, t8598: f64, t12291: f64, t7056: f64, t10091: f64, t31783: f64, t12058: f64, t4908: f64, t10099: f64, t10791: f64, t12285: f64, t7063: f64, t972: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36455 = 2.0_f64 * t8598 * t3537;
    let t36457 = 4.0_f64 * t7056 * t12291;
    let t36460 = 6.0_f64 * t31783 * t10091;
    let t36462 = 4.0_f64 * t4908 * t12058;
    let t36467 = 4.0_f64 * t10099 * t10791;
    let t36470 = 12.0_f64 * t7063 * t12285 * t972;
    (t36455, t36457, t36460, t36462, t36467, t36470)
}
