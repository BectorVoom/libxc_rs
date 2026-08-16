//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1067/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1067(t10820: f64, t2301: f64, t3928: f64, t5218: f64, t645: f64, t25918: f64, t8548: f64, t4044: f64, t5184: f64, t1632: f64, t3352: f64, t495: f64, t511: f64, t7230: f64) -> (f64, f64, f64, f64, f64) {
    let t42059 = t10820 * t2301;
    let t42066 = t3928 * t645 * t5218;
    let t42068 = t25918 * t8548;
    let t42071 = t4044 * t645 * t5184;
    let t42076 = t7230 * t3352 * t511 * t1632 * t495;
    (t42059, t42066, t42068, t42071, t42076)
}
