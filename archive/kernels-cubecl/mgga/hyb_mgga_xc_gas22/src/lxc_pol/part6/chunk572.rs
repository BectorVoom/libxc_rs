//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 572/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk572<F: Float>(t10: F, t1067: F, t1107: F, t1030: F, t567: F, t1048: F, t222: F, t16: F, t1884: F, t492: F) -> (F, F, F, F, F) {
    let t2699 = t1067 * t10;
    let t2700 = t2699 * t1107;
    let t2702 = t567 * t1030;
    let t2705 = F::cast_from(0.35616666666666666666e-1_f64) * t222 * t2702 * t1048;
    let t2707 = t16 * t1884 * t492;
    (t2699, t2700, t2702, t2705, t2707)
}
