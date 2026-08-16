//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 841/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk841(t12830: f64, t1422: f64, t3533: f64, t1365: f64, t3619: f64, t5953: f64, t1056: f64, t1390: f64, t3283: f64) -> (f64, f64, f64, f64) {
    let t12860 = t1422 * t3533 * t12830;
    let t12863 = t1365 * t3619;
    let t12864 = t5953 * t12863;
    let t12867 = t1390 * t1056;
    let t12868 = t12867 * t3283;
    (t12860, t12863, t12864, t12868)
}
