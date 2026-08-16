//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 614/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk614(t1045: f64, t4848: f64, t1035: f64, t4670: f64, t102: f64, t2474: f64, t1056: f64, t4621: f64, t159: f64, t23: f64, t6: f64, t107: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4849 = t4848 * t1045;
    let t4852 = t1035 * t4670;
    let t4858 = t102 * t2474;
    let t4859 = t1056 * t4621;
    let t4863 = 1.0_f64 / t23 / t159;
    let t4864 = t6 * t4863;
    let t4865 = t107 * t4864;
    (t4849, t4852, t4858, t4859, t4863, t4864, t4865)
}
