//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1382/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1382<F: Float>(t1799: F, t22290: F, t9679: F, t112576: F, t117031: F, t117033: F, t121796: F, t121800: F, t121803: F, t121806: F, t121809: F, t121812: F, t121815: F, t32990: F, t34073: F, t34078: F, t34148: F, t34154: F, t35136: F) -> (F, F) {
    let t121818 = t1799 * t9679 * t22290;
    let t121826 = 0.10416666666666666667e-1 * t32990 * t35136 - 0.24872916666666666666e-2 * t121796 - 0.55273148148148148147e-3 * t112576 + t117031 + t117033 + 0.17687407407407407407e-1 * t121800 - 0.14739506172839506172e-1 * t121803 + 0.22109259259259259258e-2 * t121806 - 0.44218518518518518516e-2 * t121809 + 0.33163888888888888888e-2 * t121812 - 0.22109259259259259259e-2 * t121815 + 0.99491666666666666664e-2 * t121818 - 0.20833333333333333334e-1 * t34073 * t34148 - 0.41666666666666666668e-1 * t34073 * t34078 - 0.24125000000000000001e-1 * t34154 * t34078;
    (t121818, t121826)
}
