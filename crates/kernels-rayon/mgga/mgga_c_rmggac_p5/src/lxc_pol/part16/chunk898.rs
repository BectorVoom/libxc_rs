//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 898/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk898(t2415: f64, t39553: f64, t7349: f64, t1525: f64, t236: f64, t3352: f64, t551: f64, t7230: f64, t1587: f64, t615: f64, t10044: f64, t1982: f64, t7428: f64) -> (f64, f64, f64, f64) {
    let t44894 = t7349 * t2415 * t39553;
    let t44901 = t7230 * t3352 * t236 * t551 * t1525;
    let t44906 = t7230 * t3352 * t236 * t1587 * t615;
    let t44909 = t10044 * t7428 * t1982;
    (t44894, t44901, t44906, t44909)
}
