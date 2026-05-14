//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1102/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1102<F: Float>(t31632: F, t31644: F, t35882: F, t35885: F, t35890: F, t35891: F, t35893: F, t35894: F, t35898: F, t35904: F, t37777: F, t37778: F, t37779: F, t40295: F, t40297: F, t40299: F, t40301: F) -> (F,) {
    let t42034 = -t35882 / 32.0 - t35885 / 96.0 + t35890 + t35891 + t35893 + t35894 + t35898 - 0.80031500487063509014e-2 * t31632 - t40295 / 32.0 + 0.17149607247227894789e-1 * t40297 - t40299 / 24.0 - t40301 / 24.0 - 0.22675591804667994221e-1 * t31644 - t35904 + t37777 + t37778 + t37779;
    (t42034,)
}
