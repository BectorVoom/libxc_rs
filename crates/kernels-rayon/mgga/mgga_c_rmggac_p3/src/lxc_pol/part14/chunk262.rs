//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 262/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk262(t472: f64, t998: f64, t1201: f64, t1206: f64, t206: f64, t207: f64, t470: f64, t473: f64) -> (f64, f64) {
    let t1209 = t472 * t998;
    let t1212 = -t1201 * t207 - 12.0_f64 * t1206 * t206 + 3.0_f64 * t1209 * t206 + 6.0_f64 * t470 * t473;
    (t1209, t1212)
}
