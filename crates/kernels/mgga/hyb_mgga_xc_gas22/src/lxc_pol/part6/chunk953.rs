//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 953/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk953<F: Float>(t6648: F, t8670: F, t8673: F, t8676: F, t8683: F, t8685: F, t8689: F, t8692: F, t8695: F, t8699: F, t8703: F, t8706: F) -> F {
    let t8708 = F::cast_from(0.19419375e1_f64) * t8670 - F::cast_from(0.412621875e-1_f64) * t8673 + F::cast_from(0.40256666666666666667e0_f64) * t8676 + F::cast_from(0.258925e1_f64) * t8683 + F::cast_from(0.16504875e0_f64) * t8685 - t6648 - t8689 - t8692 + F::cast_from(0.248355e0_f64) * t8695 + F::cast_from(0.49671e0_f64) * t8699 + F::cast_from(0.248355e0_f64) * t8703 + F::cast_from(0.27595e0_f64) * t8706;
    t8708
}
