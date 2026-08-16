//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 799/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk799(t1494: f64, t209: f64, t26: f64, t14163: f64, t2067: f64, t3369: f64, t3352: f64, t495: f64, t515: f64, t7230: f64, t8975: f64, t15405: f64, t7255: f64) -> (f64, f64, f64) {
    let t74411 = t26 * t1494 * t209;
    let t74414 = t14163 * t3369 * t2067 * t74411;
    let t74419 = t7230 * t3352 * t515 * t8975 * t495;
    let t74421 = t7255 * t15405;
    (t74414, t74419, t74421)
}
