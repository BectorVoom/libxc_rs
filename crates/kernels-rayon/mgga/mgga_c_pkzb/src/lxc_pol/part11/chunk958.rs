//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 958/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk958(t1058: f64, t3501: f64, t8771: f64, t1020: f64, t3401: f64) -> (f64, f64, f64) {
    let t10496 = t3501 * t1058;
    let t10501 = 0.17544670867903938621e1_f64 * t8771;
    let t10502 = t3401 * t1020;
    (t10496, t10501, t10502)
}
