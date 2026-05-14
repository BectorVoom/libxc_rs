//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1299/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1299<F: Float>(t109767: F, t109787: F, t122592: F, t122647: F, t123814: F, t124895: F, t125061: F, t1403: F, t1454: F, t17713: F, t193: F, t263: F, t27908: F, t27997: F, t28015: F, t30862: F, t30900: F, t31063: F, t41409: F, t5996: F, t6745: F, t675: F, t766: F) -> (F,) {
    let t125296 = -t109767 + t6745 * t27908 / 3.0 + 8.0 * t122592 - t17713 * t1454 + 4.0 / 27.0 * t109787 + 4.0 * t125061 + t5996 * t30862 / 6.0 + t1403 * t193 * t675 * t123814 * t263 / 6.0 + 48.0 * t41409 * t31063 * t766 + t5996 * t30900 / 3.0 + 2.0 * t28015 * t27997 - 12.0 * t124895 - 12.0 * t122647;
    (t125296,)
}
