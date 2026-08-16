//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 799/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk799(t1476: f64, t236: f64, t14117: f64, t68906: f64, t69839: f64, t9146: f64, t14124: f64, t201: f64, t457: f64, t618: f64, t68422: f64, t14131: f64, t9170: f64) -> (f64, f64, f64, f64, f64) {
    let t74376 = t236 * t1476;
    let t74378 = t68906 * t14117 * t74376;
    let t74381 = t69839 * t14117 * t9146;
    let t74387 = t14124 * t68422 * t236 * t618 * t457 * t201;
    let t74390 = t14131 * t68422 * t9170;
    (t74376, t74378, t74381, t74387, t74390)
}
