//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1089/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1089<F: Float>(t31224: F, t32739: F, t32740: F, t35418: F, t35424: F, t37527: F, t37528: F, t37529: F, t37531: F, t37533: F, t37534: F, t39937: F, t39939: F, t39944: F, t39946: F, t39948: F, t39950: F, t39952: F) -> (F,) {
    let t41866 = -0.68598428988911579156e-2 * t39937 - 0.94344276868812456204e-2 * t39939 + t37527 - t37528 + t37529 + t37531 - 0.62896184579208304136e-2 * t39944 - 0.38586616306262763275e-1 * t39946 + 0.80031500487063509015e-2 * t39948 - t37533 - t37534 + 0.68598428988911579156e-2 * t39950 + 0.12862205435420921092e-2 * t39952 - 0.45017719023973223822e-1 * t31224 + t32739 + 0.13208198761633743869e0 * t35418 + t32740 + t35424;
    (t41866,)
}
