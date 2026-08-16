//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 881/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk881(t290: f64, t38843: f64, t2012: f64, t7349: f64, t2412: f64, t7424: f64, t7421: f64, t36639: f64, t8636: f64, t4968: f64, t511: f64, t2344: f64) -> (f64, f64, f64, f64, f64) {
    let t39553 = t290 * t38843;
    let t39555 = t7349 * t2012 * t39553;
    let t39559 = t2412 * t7424;
    let t39561 = t2412 * t7421;
    let t39563 = t36639 * t8636;
    let t39565 = t4968 * t511;
    let t39566 = t39565 * t2344;
    (t39555, t39559, t39561, t39563, t39566)
}
