//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2415/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2415<F: Float>(t12256: F, t3698: F, t3362: F, t414: F, t12884: F, t828: F, t3555: F, t3766: F, t5330: F, t1209: F, t13147: F, t17708: F) -> (F, F, F, F, F) {
    let t44348 = t3698 * t12256;
    let t44361 = F::cast_from(1.0_f64) / t414 / t3362;
    let t44425 = t828 * t12884;
    let t44484 = t3555 * t3766 * t5330;
    let t44500 = t1209 * t13147 * t17708;
    (t44348, t44361, t44425, t44484, t44500)
}
