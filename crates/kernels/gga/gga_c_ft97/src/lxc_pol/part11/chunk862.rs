//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 862/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk862<F: Float>(t126: F, t37627: F, t120: F, t37640: F, t7977: F, t32: F, t7911: F, t8991: F, t123: F, t37993: F, t532: F, t1655: F, t2022: F, t535: F, t1580: F, t8965: F) -> (F, F, F, F, F, F, F, F, F) {
    let t39861 = t37627 * t126;
    let t39866 = t37640 * t120;
    let t39869 = t120 * t7977;
    let t39872 = t37640 * t126;
    let t39877 = t8991 / t32 / t7911;
    let t39889 = t123 / t532 / t37993;
    let t39892 = t2022 * t1655;
    let t39895 = t535 * t7977;
    let t39907 = t8965 * t1580;
    (t39861, t39866, t39869, t39872, t39877, t39889, t39892, t39895, t39907)
}
