//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1372/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1372<F: Float>(t3485: F, t9135: F, t29757: F, t29760: F, t29788: F, t29822: F, t29825: F, t29827: F, t29833: F, t29836: F, t29839: F, t29842: F, t29844: F) -> (F, F) {
    let t29846 = t3485 * t9135;
    let t29848 = F::cast_from(0.16504875e0_f64) * t29822 - F::cast_from(0.258925e1_f64) * t29825 + F::cast_from(0.16504875e0_f64) * t29827 + F::cast_from(0.40256666666666666667e0_f64) * t29757 - F::cast_from(0.60385e0_f64) * t29760 + F::cast_from(0.905775e0_f64) * t29788 - F::cast_from(0.485484375e1_f64) * t29833 + F::cast_from(0.19419375e1_f64) * t29836 + F::cast_from(0.6189328125e-1_f64) * t29839 - F::cast_from(0.412621875e-1_f64) * t29842 + F::cast_from(0.19419375e1_f64) * t29844 - F::cast_from(0.258925e1_f64) * t29846;
    (t29846, t29848)
}
