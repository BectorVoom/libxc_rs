//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1093/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1093<F: Float>(t10658: F, t809: F, t6562: F, t3419: F, t3435: F, t4180: F, t6640: F, t3444: F, t2289: F, t4193: F, t849: F, t260: F, t4175: F) -> (F, F, F, F, F, F, F, F) {
    let t10659 = t10658 * t809;
    let t10661 = F::new(0.51726012919273400301e3) * t6562 * t10659;
    let t10662 = t3435 * t3419;
    let t10667 = t6640 * t4180;
    let t10668 = t10667 * t3444;
    let t10671 = t2289 * t4193;
    let t10672 = t10671 * t849;
    let t10679 = t260 * t4175;
    (t10659, t10661, t10662, t10667, t10668, t10671, t10672, t10679)
}
