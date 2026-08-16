//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1094/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1094<F: Float>(t10534: F, t10549: F, t10567: F, t10569: F, t10572: F, t10578: F, t10585: F, t10587: F, t6530: F, t6648: F, t8652: F, t8676: F) -> F {
    let t10692 = F::cast_from(0.19419375e1_f64) * t10567 - F::cast_from(0.258925e1_f64) * t10569 - F::cast_from(0.1294625e1_f64) * t10572 + F::cast_from(0.258925e1_f64) * t10578 - t6648 + F::cast_from(0.40256666666666666667e0_f64) * t6530 + F::cast_from(0.80513333333333333333e0_f64) * t8676 - t8652 - F::cast_from(0.301925e0_f64) * t10534 + F::cast_from(0.905775e0_f64) * t10549 - F::cast_from(0.412621875e-1_f64) * t10585 + F::cast_from(0.16504875e0_f64) * t10587;
    t10692
}
