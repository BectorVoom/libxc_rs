//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2814/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2814<F: Float>(t10073: F, t14537: F, t51653: F, t51657: F, t51660: F, t51668: F, t51672: F, t51676: F, t51680: F, t51683: F, t51685: F, t51686: F) -> F {
    let t51688 = t10073 * t14537;
    let t51690 = F::cast_from(0.32927245914677557992e-1_f64) * t51653 - F::cast_from(0.7805952431506226415e-1_f64) * t51657 + F::cast_from(0.19637199382202157274e-3_f64) * t51660 - F::cast_from(0.17563392970889009433e0_f64) * t51668 + F::cast_from(0.16463622957338778996e-1_f64) * t51672 - F::cast_from(0.19637199382202157274e-3_f64) * t51676 + F::cast_from(0.32927245914677557992e-1_f64) * t51680 + t51683 - t51685 + F::cast_from(0.17073386770573548589e-1_f64) * t51686 + F::cast_from(0.19514881078765566037e-2_f64) * t51688;
    t51690
}
