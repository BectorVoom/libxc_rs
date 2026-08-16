//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 58/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk58(t143: f64, t158: f64, t165: f64, t167: f64, t173: f64) -> f64 {
    let t175 = -0.59778596625315888114e-2_f64 * t143 + 0.1317375e-2_f64 * t158 - 0.23775e-3_f64 * t165 + 0.64744236347453835951e-5_f64 * t167 - 0.540140625e-6_f64 * t173;
    t175
}
