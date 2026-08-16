//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1236/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1236(t2187: f64, t3313: f64, t1333: f64, t6578: f64, t2233: f64, t6561: f64, t787: f64, t8723: f64, t260: f64, t8753: f64, t6641: f64, t2250: f64, t3363: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t24699 = t3313 * t2187;
    let t24702 = t1333 * t6578;
    let t24709 = t3313 * t2233;
    let t24712 = t1333 * t6561;
    let t24774 = t8723 * t787;
    let t24788 = t260 * t8753;
    let t24799 = t260 * t6641;
    let t24813 = t3363 * t2250;
    (t24699, t24702, t24709, t24712, t24774, t24788, t24799, t24813)
}
