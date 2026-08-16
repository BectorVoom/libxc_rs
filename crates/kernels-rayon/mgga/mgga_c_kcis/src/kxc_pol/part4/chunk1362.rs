//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1362/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1362(t12520: f64, t492: f64, t15973: f64, t6028: f64, t2051: f64, t4307: f64, t16751: f64, t577: f64, t1548: f64, t16622: f64, t4288: f64, t2042: f64, t4269: f64) -> (f64, f64, f64, f64, f64) {
    let t17508 = t12520 * t492;
    let t17509 = t6028 * t15973;
    let t17510 = t17508 * t17509;
    let t17512 = t2051 * t4307;
    let t17514 = t16751 * t577;
    let t17515 = t17514 * t1548;
    let t17517 = t16622 * t577;
    let t17518 = t17517 * t4288;
    let t17520 = t2042 * t4269;
    (t17510, t17512, t17515, t17518, t17520)
}
