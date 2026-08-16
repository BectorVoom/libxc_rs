//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1199/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1199(t4028: f64, t8327: f64, t7458: f64, t1774: f64, t8326: f64, t652: f64, t1842: f64, t31090: f64, t22635: f64, t1992: f64, t6906: f64, t7749: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32673 = t4028 * t8327;
    let t32674 = 2.0_f64 * t32673;
    let t32675 = t7458 * t8327;
    let t32676 = 2.0_f64 * t32675;
    let t32677 = t1774 * t8326;
    let t32678 = t652 * t32677;
    let t32679 = 2.0_f64 * t32678;
    let t32693 = t31090 * t1842;
    let t32694 = t22635 * t32693;
    let t32696 = 0.3289868133696452873e-1_f64 * t1992 * t32694;
    let t32697 = t6906 * t7749;
    (t32674, t32676, t32677, t32679, t32693, t32694, t32696, t32697)
}
