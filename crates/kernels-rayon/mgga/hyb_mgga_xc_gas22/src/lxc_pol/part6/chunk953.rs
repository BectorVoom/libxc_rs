//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 953/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk953(t6648: f64, t8670: f64, t8673: f64, t8676: f64, t8683: f64, t8685: f64, t8689: f64, t8692: f64, t8695: f64, t8699: f64, t8703: f64, t8706: f64) -> f64 {
    let t8708 = 0.19419375e1_f64 * t8670 - 0.412621875e-1_f64 * t8673 + 0.40256666666666666667e0_f64 * t8676 + 0.258925e1_f64 * t8683 + 0.16504875e0_f64 * t8685 - t6648 - t8689 - t8692 + 0.248355e0_f64 * t8695 + 0.49671e0_f64 * t8699 + 0.248355e0_f64 * t8703 + 0.27595e0_f64 * t8706;
    t8708
}
