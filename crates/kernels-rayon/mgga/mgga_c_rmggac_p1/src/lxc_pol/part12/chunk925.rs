//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 925/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk925(t1212: f64, t1970: f64, t209: f64, t236: f64, t618: f64, t7231: f64, t495: f64, t511: f64, t7230: f64, t8502: f64, t1540: f64, t2144: f64) -> (f64, f64, f64) {
    let t39946 = t1970 * t7231 * t236 * t618 * t1212 * t209;
    let t39951 = t7230 * t7231 * t511 * t8502 * t495;
    let t39953 = t1540 * t2144;
    (t39946, t39951, t39953)
}
