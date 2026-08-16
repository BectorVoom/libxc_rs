//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 950/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk950(t1336: f64, t6585: f64, t2195: f64, t6601: f64, t1329: f64, t1885: f64, t222: f64) -> (f64, f64, f64, f64, f64) {
    let t8669 = t6585 * t1336;
    let t8670 = t8669 * t2195;
    let t8672 = t6601 * t1336;
    let t8673 = t8672 * t2195;
    let t8676 = t222 * t1885 * t1329;
    (t8669, t8670, t8672, t8673, t8676)
}
