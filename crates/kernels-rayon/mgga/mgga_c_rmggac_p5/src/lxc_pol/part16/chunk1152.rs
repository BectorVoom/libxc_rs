//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1152/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1152(t10318: f64, t10319: f64, t10320: f64, t10321: f64, t42459: f64, t42460: f64, t42461: f64, t42462: f64, t42463: f64, t42464: f64, t42465: f64, t10325: f64, t10329: f64, t10330: f64, t42474: f64, t42475: f64, t42476: f64, t42477: f64, t42478: f64, t42484: f64, t42485: f64, t42486: f64) -> (f64, f64) {
    let t49842 = -t10318 - t42459 + t10319 - t10320 - t10321 - t42460 + t42461 + t42462 - t42463 - t42464 - t42465;
    let t49845 = -t42474 + t42475 + t42476 + t42477 - t42478 - t42484 - t42485 - t42486 + t10325 + t10329 + t10330;
    (t49842, t49845)
}
