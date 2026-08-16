//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 489/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk489(t13957: f64, t7577: f64, t739: f64, t3046: f64, t4789: f64, t71: f64) -> (f64, f64, f64, f64) {
    let t13958 = t7577 * t13957;
    let t13959 = t739 * t13958;
    let t13960 = 0.5987120850931904282e-1_f64 * t13959;
    let t13961 = t3046 * t4789;
    let t13962 = t13961 * t71;
    (t13958, t13960, t13961, t13962)
}
