//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3183/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3183<F: Float>(t43766: F, t44361: F, t12916: F, t17419: F, t5340: F, t45608: F, t58919: F, t45786: F, t17708: F, t45846: F, t12975: F, t1803: F) -> (F, F, F, F, F, F) {
    let t58983 = t44361 * t43766;
    let t58997 = t5340 * t12916 * t17419;
    let t59001 = t45608 * t58919;
    let t59011 = t45786 * t58919;
    let t59017 = t45846 * t17708;
    let t59025 = t12975 * t1803;
    (t58983, t58997, t59001, t59011, t59017, t59025)
}
