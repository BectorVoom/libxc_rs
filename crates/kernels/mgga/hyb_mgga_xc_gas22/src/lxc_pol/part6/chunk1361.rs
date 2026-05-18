//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1361/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1361<F: Float>(t10954: F, t2478: F, t967: F, t10953: F, t2523: F, t2521: F, t11035: F, t7070: F, t11039: F, t21366: F, t8983: F, t9258: F) -> (F, F, F, F, F) {
    let t29644 = F::new(4.0) * t2478 * t10954 * t967;
    let t29645 = t10953 * t2523;
    let t29648 = F::new(0.32163958997385070134e2) * t2521 * t29645 * t967;
    let t29650 = F::new(0.64327917994770140268e2) * t7070 * t11035;
    let t29652 = F::new(0.1034520258385468006e4) * t21366 * t11039;
    let t29654 = F::new(4.0) * t9258 * t8983;
    (t29644, t29648, t29650, t29652, t29654)
}
