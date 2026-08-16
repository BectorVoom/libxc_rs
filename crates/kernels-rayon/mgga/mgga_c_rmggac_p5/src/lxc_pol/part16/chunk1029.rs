//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1029/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1029(t3928: f64, t6441: f64, t645: f64, t4044: f64, t6421: f64, t2060: f64, t45622: f64, t903: f64, t34847: f64, t9971: f64, t1614: f64, t1971: f64, t511: f64, t615: f64, t7230: f64) -> (f64, f64, f64, f64, f64) {
    let t47487 = t3928 * t645 * t6441;
    let t47490 = t4044 * t645 * t6421;
    let t47493 = t903 * t2060 * t45622;
    let t47495 = t34847 * t9971;
    let t47500 = t7230 * t1971 * t511 * t1614 * t615;
    (t47487, t47490, t47493, t47495, t47500)
}
