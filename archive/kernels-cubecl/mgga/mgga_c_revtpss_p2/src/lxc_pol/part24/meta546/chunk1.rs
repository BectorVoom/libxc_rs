//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1619/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1619<F: Float>(t6002: F, t61037: F, t61180: F, t76979: F, t39791: F, t39795: F, t39799: F, t39807: F, t39813: F, t39818: F, t39823: F, t40084: F) -> (F, F, F, F) {
    let t87649 = F::cast_from(72.0_f64) * t61037 * t6002;
    let t87650 = F::cast_from(48.0_f64) * t61180;
    let t87651 = F::cast_from(48.0_f64) * t76979;
    let t87652 = -t39791 - t39795 + t87649 + t39799 + t39807 - t39813 + t87650 - t39818 - t39823 + t40084 + t87651;
    (t87649, t87650, t87651, t87652)
}
