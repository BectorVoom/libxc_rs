//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1004/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1004(t2092: f64, t25894: f64, t3677: f64, t1471: f64, t30294: f64, t12: f64) -> (f64, f64) {
    let t30565 = t25894 * t2092;
    let t30567 = 0.48245472966453314466e2_f64 * t3677 * t30565;
    let t30568 = t1471 * t30294;
    let t30569 = t12 * t30568;
    (t30567, t30569)
}
