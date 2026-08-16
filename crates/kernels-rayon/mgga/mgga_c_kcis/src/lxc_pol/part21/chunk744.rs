//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 744/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk744(t8067: f64, t8070: f64, t8073: f64, t8075: f64, t8077: f64, t8079: f64) -> f64 {
    let t8117 = 0.9375e-1_f64 * t8067 - 0.9375e-1_f64 * t8070 + 0.625e-1_f64 * t8073 - 0.20234375e-1_f64 * t8075 + 0.20234375e-1_f64 * t8077 - 0.26979166666666666667e-1_f64 * t8079;
    t8117
}
