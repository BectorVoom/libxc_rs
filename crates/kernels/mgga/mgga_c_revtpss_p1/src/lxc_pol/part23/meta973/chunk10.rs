//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3308/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3308<F: Float>(t1448: F, t6836: F, t13600: F, t22466: F, t39799: F, t39807: F, t39813: F, t4139: F, t47059: F, t48271: F, t5536: F, t5627: F, t6816: F, t85913: F, t85914: F, t85918: F, t85919: F) -> (F, F) {
    let t86753 = t6836 * t1448;
    let t86764 = F::new(9.0) * t13600 * t4139 * t6816 - F::new(18.0) * t22466 * t5536 * t5627 + t39799 + t39807 - t39813 + t47059 + t48271 - t85913 + t85914 - t85918 - t85919;
    (t86753, t86764)
}
