//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1094/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1094(t10534: f64, t10549: f64, t10567: f64, t10569: f64, t10572: f64, t10578: f64, t10585: f64, t10587: f64, t6530: f64, t6648: f64, t8652: f64, t8676: f64) -> f64 {
    let t10692 = 0.19419375e1_f64 * t10567 - 0.258925e1_f64 * t10569 - 0.1294625e1_f64 * t10572 + 0.258925e1_f64 * t10578 - t6648 + 0.40256666666666666667e0_f64 * t6530 + 0.80513333333333333333e0_f64 * t8676 - t8652 - 0.301925e0_f64 * t10534 + 0.905775e0_f64 * t10549 - 0.412621875e-1_f64 * t10585 + 0.16504875e0_f64 * t10587;
    t10692
}
