//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1108/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1108<F: Float>(t15483: F, t2615: F, t9438: F, t7416: F, t9830: F, t10029: F, t2464: F, t2465: F, t2684: F, t7258: F, t22424: F, t3311: F) -> (F, F, F, F, F) {
    let t28818 = t2615 * t9438 * t15483;
    let t28820 = t7416 * t9830;
    let t28822 = t7416 * t10029;
    let t28827 = F::cast_from(0.17041300423964777634e0_f64) * t2684 * t2464 * t2465 * t7258;
    let t28828 = t22424 * t3311;
    (t28818, t28820, t28822, t28827, t28828)
}
