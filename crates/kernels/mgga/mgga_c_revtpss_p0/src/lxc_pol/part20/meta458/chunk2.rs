//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1748/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1748<F: Float>(t47078: F, t39807: F, t39813: F, t47057: F, t47059: F, t47061: F, t47064: F, t47067: F, t47070: F, t47072: F, t47074: F, t47076: F) -> (F, F) {
    let t47079 = F::cast_from(0.73245789224026180216e-3_f64) * t47078;
    let t47080 = t47057 + t47059 + t47061 + t39807 - t39813 + t47064 + t47067 + t47070 - t47072 + t47074 - t47076 - t47079;
    (t47079, t47080)
}
