//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1002/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1002(t30294: f64, t6175: f64, t30153: f64, t3953: f64, t1312: f64, t13878: f64, t3952: f64, t13895: f64, t13894: f64, t2105: f64, t7802: f64) -> (f64, f64, f64, f64, f64) {
    let t30536 = t6175 * t30294;
    let t30539 = t3953 * t30153;
    let t30540 = t1312 * t30539;
    let t30543 = t13878 * t30153;
    let t30544 = t3952 * t30543;
    let t30547 = t13895 * t30153;
    let t30548 = t13894 * t30547;
    let t30551 = t7802 * t2105;
    (t30536, t30540, t30544, t30548, t30551)
}
