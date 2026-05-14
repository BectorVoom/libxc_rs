//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 650/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk650<F: Float>(t12962: F, t12969: F, t12989: F, t12992: F, t12998: F, t13808: F, t13811: F, t13815: F, t13820: F, t13824: F, t13828: F, t13832: F, t14458: F, t14463: F, t14472: F, t502: F) -> (F, F) {
    let t14477 = -0.21450293971110256002e1 * t13808 + 0.14300195980740170668e1 * t13811 - 0.13803453343411469884e2 * t13815 + t12962 - 0.89376224879626066674e-1 * t12969 - t13820 - t13824 + t13828 + t13832 + t12989 + t12992 + t12998;
    let t14479 = t14458 + t14463 + t14472 + t14477;
    let t14480 = t502 * t14479;
    (t14479, t14480)
}
