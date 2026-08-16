//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1031/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1031(t2931: f64, t852: f64, t2935: f64, t932: f64, t3063: f64, t177: f64, t3042: f64, t140: f64, t191: f64, t3043: f64, t912: f64, t3032: f64, t919: f64) -> (f64, f64, f64, f64, f64) {
    let t15217 = t852 * t2931;
    let t15220 = t932 * t2935;
    let t15221 = t15220 * t3063;
    let t15224 = t3042 * t177;
    let t15226 = t140 * t15224 * t191;
    let t15232 = t912 * t3043;
    let t15237 = t3032 * t919;
    (t15217, t15221, t15226, t15232, t15237)
}
