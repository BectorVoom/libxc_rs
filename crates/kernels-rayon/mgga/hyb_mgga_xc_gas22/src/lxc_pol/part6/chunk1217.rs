//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1217/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1217(t2970: f64, t7848: f64, t7861: f64, t240: f64, t6184: f64, t92: f64, t7843: f64, t7866: f64, t639: f64, t7867: f64, t1804: f64, t6214: f64, t8211: f64) -> (f64, f64, f64, f64, f64) {
    let t23696 = t2970 * t7848 * t7861;
    let t23699 = t240 * t6184 * t92;
    let t23701 = t7866 * t23699 * t7843;
    let t23706 = t7867 * t639;
    let t23726 = t1804 * t6214 * t8211;
    (t23696, t23699, t23701, t23706, t23726)
}
