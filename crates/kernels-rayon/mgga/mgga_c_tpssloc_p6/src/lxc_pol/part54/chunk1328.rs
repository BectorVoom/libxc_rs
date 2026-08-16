//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1328/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1328(t19456: f64, t8326: f64, t26114: f64, t26117: f64, t12725: f64, t22751: f64, t32731: f64, t22633: f64, t22635: f64, t31099: f64, t5187: f64, t31090: f64, t97721: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t120120 = t19456 * t8326;
    let t120122 = t26114 * t8326;
    let t120124 = t26117 * t8326;
    let t120130 = t12725 * t8326;
    let t120179 = t22751 * t32731;
    let t120180 = 0.76763589786250567037e-1_f64 * t120179;
    let t120184 = 0.3289868133696452873e-1_f64 * t22633 * t22635 * t31099 * t5187;
    let t120196 = 0.6579736267392905746e-1_f64 * t22633 * t22635 * t31090 * t97721;
    (t120120, t120122, t120124, t120130, t120180, t120184, t120196)
}
