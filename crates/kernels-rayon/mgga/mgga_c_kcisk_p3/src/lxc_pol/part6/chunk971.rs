//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 971/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk971(t2670: f64, t8472: f64, t564: f64, t22167: f64, t2356: f64, t8473: f64, t2059: f64, t7706: f64) -> (f64, f64, f64, f64) {
    let t30146 = t8472 * t2670;
    let t30147 = t564 * t30146;
    let t30148 = 3.0_f64 / 16.0_f64 * t30147;
    let t30149 = 3.0_f64 * t22167;
    let t30150 = t2356 * t8473;
    let t30151 = 3.0_f64 / 16.0_f64 * t30150;
    let t30153 = t7706 * t2059;
    (t30148, t30149, t30151, t30153)
}
