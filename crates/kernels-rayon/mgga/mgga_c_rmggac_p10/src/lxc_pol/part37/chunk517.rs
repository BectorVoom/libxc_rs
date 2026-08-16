//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 517/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk517(t14165: f64, t2067: f64, t3369: f64, t14163: f64, t3065: f64, t333: f64) -> (f64, f64, f64) {
    let t14166 = t2067 * t14165;
    let t14167 = t3369 * t14166;
    let t14168 = t14163 * t14167;
    let t14170 = t3065 * t333;
    (t14167, t14168, t14170)
}
