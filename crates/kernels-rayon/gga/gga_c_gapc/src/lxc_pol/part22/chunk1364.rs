//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1364/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1364(t10091: f64, t31783: f64, t12058: f64, t4908: f64, t1615: f64, t3804: f64, t1617: f64, t10099: f64, t10791: f64, t12285: f64, t7063: f64, t972: f64) -> (f64, f64, f64, f64, f64) {
    let t36460 = 6.0_f64 * t31783 * t10091;
    let t36462 = 4.0_f64 * t4908 * t12058;
    let t36463 = t3804 * t1615;
    let t36465 = 2.0_f64 * t36463 * t1617;
    let t36467 = 4.0_f64 * t10099 * t10791;
    let t36470 = 12.0_f64 * t7063 * t12285 * t972;
    (t36460, t36462, t36465, t36467, t36470)
}
