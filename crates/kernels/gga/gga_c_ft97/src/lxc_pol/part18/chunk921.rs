//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 921/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk921<F: Float>(t23898: F, t23923: F, t23669: F, t23674: F, t23887: F, t23890: F, t23895: F, t23903: F, t23907: F, t23912: F, t23914: F, t23918: F, t23920: F, t23928: F, t23931: F, t23935: F) -> (F, F, F) {
    let t24034 = 2.0 / 27.0 * t23898;
    let t24041 = 4.0 / 27.0 * t23923;
    let t24045 = -2.0 / 9.0 * t23669 - t23674 / 18.0 - t23887 / 6.0 + t23890 / 9.0 - 2.0 / 9.0 * t23895 - t24034 + 2.0 / 9.0 * t23903 + t23907 / 9.0 + 2.0 / 27.0 * t23912 - 2.0 / 27.0 * t23914 - t23918 / 3.0 + 2.0 / 9.0 * t23920 - t24041 + 4.0 / 3.0 * t23928 - 4.0 / 9.0 * t23931 + 2.0 / 3.0 * t23935;
    (t24034, t24041, t24045)
}
