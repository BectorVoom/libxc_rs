//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 528/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk528(t10649: f64, t2067: f64, t14243: f64, t14236: f64, t1985: f64, t838: f64) -> (f64, f64, f64) {
    let t14244 = t2067 * t10649;
    let t14245 = t14243 * t14244;
    let t14246 = t14236 * t14245;
    let t14249 = t1985 * t838;
    (t14245, t14246, t14249)
}
