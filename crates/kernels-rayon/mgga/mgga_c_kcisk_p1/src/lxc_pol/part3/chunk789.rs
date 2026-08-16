//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 789/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk789(t12179: f64, t2013: f64, t4998: f64, t5502: f64, t10886: f64, t5487: f64, t5471: f64, t5480: f64, t5464: f64, t1772: f64, t10487: f64, t786: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12180 = t2013 * t12179;
    let t12182 = t4998 * t5502;
    let t12183 = t2013 * t12182;
    let t12185 = t10886 * t5487;
    let t12186 = t2013 * t12185;
    let t12188 = t5471 * t5480;
    let t12194 = t5464 * sigma2;
    let t12195 = t12194 * t1772;
    let t12198 = t786 * t10487;
    (t12180, t12183, t12186, t12188, t12195, t12198)
}
