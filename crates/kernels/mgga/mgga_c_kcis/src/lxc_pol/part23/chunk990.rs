//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 990/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk990<F: Float>(t27410: F, t27459: F, t28353: F, t28388: F, t28392: F, t28395: F, t28397: F, t28403: F, t28407: F, t28410: F, t28413: F, t28415: F, t28420: F, t28424: F, t28427: F, t28430: F, t28439: F, t28443: F, t7895: F, t7898: F, t7901: F, t7908: F, t7911: F, t7916: F, t8144: F, t8148: F, t8155: F) -> (F,) {
    let t28446 = -0.185671721767578125e-4 * t28388 * t28353 + 0.61782407407407407407e-3 * t28392 * t7911 + 0.11054629629629629629e-2 * t28395 + 0.92754700520833333333e-4 * t28397 * t7901 + 0.92754700520833333333e-4 * t27410 * t8148 + 0.92754700520833333333e-4 * t7898 * t28403 + 0.66327777777777777776e-2 * t28407 - 0.44218518518518518517e-2 * t28410 + 0.16581944444444444444e-2 * t28413 + 0.11054629629629629629e-2 * t28415 - 0.23168402777777777778e-3 * t27459 * t8155 + 0.46336805555555555556e-3 * t7908 * t28420 + 0.23168402777777777778e-3 * t28424 + 0.30918233506944444444e-4 * t28427 - 0.24872916666666666666e-2 * t28430 + 0.69505208333333333333e-3 * t8144 * t7916 + 0.69505208333333333333e-3 * t8144 * t7901 + 0.69505208333333333333e-3 * t7895 * t8148 + 0.23168402777777777778e-3 * t7908 * t28439 + 0.23168402777777777778e-3 * t7908 * t28443;
    (t28446,)
}
