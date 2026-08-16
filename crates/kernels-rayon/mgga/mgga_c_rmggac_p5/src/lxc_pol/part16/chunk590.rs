//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 590/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk590(t2060: f64, t5144: f64, t1550: f64, t5267: f64, t903: f64, t1627: f64, t645: f64, t3928: f64, t5888: f64, t7577: f64, t739: f64, t1469: f64, t236: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8404 = t2060 * t5144;
    let t8405 = t1550 * t8404;
    let t8407 = t2060 * t5267;
    let t8408 = t903 * t8407;
    let t8410 = t645 * t1627;
    let t8411 = t3928 * t8410;
    let t8413 = t7577 * t5888;
    let t8414 = t739 * t8413;
    let t8416 = t236 * t1469;
    (t8404, t8405, t8407, t8408, t8410, t8411, t8413, t8414, t8416)
}
