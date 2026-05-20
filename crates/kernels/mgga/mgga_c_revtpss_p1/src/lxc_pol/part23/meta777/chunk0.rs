//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2581/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2581<F: Float>(t58145: F, t58225: F, t3432: F, t5060: F, t12226: F, t1719: F, t56228: F, t56176: F, t56183: F, t12555: F, t5180: F, t12486: F, t300: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t58411 = F::new(0.27595e0) * t58145;
    let t58452 = F::new(0.5519e0) * t58225;
    let t58466 = t5060 * t3432;
    let t58473 = t1719 * t12226;
    let t58536 = F::cast_from(0.39862222222222222223e0_f64) * t56228;
    let t58543 = F::cast_from(0.27385555555555555556e0_f64) * t58145;
    let t58607 = F::cast_from(0.1522074074074074074e-1_f64) * t56176;
    let t58609 = F::cast_from(0.4566222222222222222e-1_f64) * t56183;
    let t58624 = F::cast_from(0.2283111111111111111e-1_f64) * t56228;
    let t58647 = t5180 * t12555;
    let t58665 = t300 * t12486;
    (t58411, t58452, t58466, t58473, t58536, t58543, t58607, t58609, t58624, t58647, t58665)
}
