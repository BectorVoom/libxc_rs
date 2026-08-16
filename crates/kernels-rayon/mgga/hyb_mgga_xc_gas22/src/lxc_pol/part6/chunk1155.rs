//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1155/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1155(t19: f64, t2986: f64, t669: f64, t1815: f64, t1862: f64, t547: f64, t5878: f64, t1056: f64, t3: f64, t1823: f64, t1816: f64, t1867: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19574 = t19 * t2986 * t669;
    let t19577 = t19 * t1815 * t1862;
    let t19579 = t547 * t5878;
    let t19643 = t3 * t1056;
    let t19664 = t19 * t1815 * t1823;
    let t19698 = t1867 * t1816;
    (t19574, t19577, t19579, t19643, t19664, t19698)
}
