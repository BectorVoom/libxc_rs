//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1172/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1172<F: Float>(t34107: F, t9680: F, t1799: F, t34073: F, t34078: F, t34081: F, t34083: F, t34087: F, t34091: F, t34095: F, t34098: F, t34101: F, t34105: F, t9664: F, t9672: F, t9660: F, t9918: F) -> (F, F, F, F) {
    let t34108 = t34107 * t9680;
    let t34109 = t1799 * t34108;
    let t34111 = 0.10416666666666666667e-1 * t34073 * t9672 - 0.20833333333333333334e-1 * t9664 * t34078 - 0.16581944444444444444e-2 * t34081 + 0.92592592592592592597e-2 * t34083 - 0.16581944444444444444e-2 * t34087 - 0.55273148148148148147e-3 * t34091 - 0.44218518518518518517e-2 * t34095 + 0.34722222222222222223e-2 * t34098 + 0.16581944444444444444e-2 * t34101 - 0.33163888888888888888e-2 * t34105 + 0.16581944444444444444e-2 * t34109;
    let t34113 = t9918 * t9660;
    (t34108, t34109, t34111, t34113)
}
