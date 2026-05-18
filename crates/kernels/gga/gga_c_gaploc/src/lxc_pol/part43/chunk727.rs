//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 727/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk727<F: Float>(t12962: F, t12969: F, t12989: F, t12992: F, t12998: F, t13808: F, t13811: F, t13815: F, t13820: F, t13824: F, t13828: F, t13832: F) -> F {
    let t14477 = -F::new(0.21450293971110256002e1) * t13808 + F::new(0.14300195980740170668e1) * t13811 - F::new(0.13803453343411469884e2) * t13815 + t12962 - F::new(0.89376224879626066674e-1) * t12969 - t13820 - t13824 + t13828 + t13832 + t12989 + t12992 + t12998;
    t14477
}
