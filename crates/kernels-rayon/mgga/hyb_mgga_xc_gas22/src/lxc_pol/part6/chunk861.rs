//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 861/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk861(t238: f64, t2507: f64, t801: f64, t2511: f64, t2558: f64, t977: f64, t365: f64) -> (f64, f64, f64, f64) {
    let t7040 = t238 * t801 * t2507;
    let t7043 = t238 * t801 * t2511;
    let t7058 = 1.0_f64 / t2558 / t977;
    let t7059 = t365 * t7058;
    (t7040, t7043, t7058, t7059)
}
