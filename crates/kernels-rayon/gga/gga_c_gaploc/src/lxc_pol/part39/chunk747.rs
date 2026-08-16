//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 747/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk747(t10012: f64, t935: f64, t9438: f64, t2684: f64, t12656: f64, t2685: f64, t10151: f64, t874: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12704 = t10012 * t935;
    let t12705 = t9438 * t12704;
    let t12706 = t2684 * t12705;
    let t12708 = t2685 * t12656;
    let t12709 = t2684 * t12708;
    let t12762 = t10151 * t874;
    (t12704, t12705, t12706, t12708, t12709, t12762)
}
