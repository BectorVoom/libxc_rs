//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1097/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1097<F: Float>(t10703: F, t839: F, t848: F, t10534: F, t10549: F, t6528: F, t6530: F, t8676: F, t8721: F, t251: F, t10567: F, t10569: F, t10572: F, t10578: F, t10585: F, t10587: F, t6691: F, t8877: F) -> (F, F, F, F) {
    let t10705 = t839 * t10703 * t848;
    let t10718 = -t6528 + F::new(0.23744444444444444444e-1) * t6530 + F::new(0.47488888888888888888e-1) * t8676 - t8721 - F::new(0.17808333333333333333e-1) * t10534 + F::new(0.53425e-1) * t10549;
    let t10720 = F::new(0.621814e-1) * t10718 * t251;
    let t10731 = F::new(0.264729375e1) * t10567 - F::new(0.3529725e1) * t10569 - F::new(0.17648625e1) * t10572 + F::new(0.3529725e1) * t10578 - t6691 + F::new(0.68863333333333333333e0) * t6530 + F::new(0.13772666666666666667e1) * t8676 - t8877 - F::new(0.516475e0) * t10534 + F::new(0.1549425e1) * t10549 - F::new(0.157790625e0) * t10585 + F::new(0.6311625e0) * t10587;
    (t10705, t10718, t10720, t10731)
}
