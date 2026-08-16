//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2011/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2011(t1984: f64, t80845: f64, t2010: f64, t6973: f64, t80742: f64, t22724: f64, t22727: f64, t22894: f64, t80670: f64, t22882: f64, t22892: f64, t22893: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81071 = t80845 * t1984;
    let t81072 = t81071 * t2010;
    let t81073 = 0.27720185200590482541e0_f64 * t81072;
    let t81074 = t80742 * t6973;
    let t81075 = 0.16220877603642232915e0_f64 * t81074;
    let t81076 = t22724 * t22727;
    let t81080 = t80670 * t22894;
    let t81083 = t22892 * t22893 * t22882;
    (t81071, t81073, t81075, t81076, t81080, t81083)
}
