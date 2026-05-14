//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1049/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1049<F: Float>(t29361: F, t5701: F, t28356: F, t8164: F, t1394: F, t5653: F, t6281: F, t7923: F, t2243: F, t7193: F, t303: F, t2237: F, t27339: F, t28465: F, t28467: F, t29324: F, t29338: F, t29341: F, t29344: F, t29355: F, t29358: F, t7908: F, t8148: F, t8151: F, t8159: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29362 = t5701 * t29361;
    let t29365 = t28356 * t8164;
    let t29366 = t1394 * t29365;
    let t29368 = t5653 * t6281;
    let t29369 = t7923 * t29368;
    let t29370 = t1394 * t29369;
    let t29372 = t7193 * t2243;
    let t29373 = t303 * t29372;
    let t29377 = -0.49745833333333333332e-2 * t29338 + 0.33163888888888888888e-2 * t29341 + 0.69505208333333333333e-3 * t2237 * t29344 - 0.37069444444444444444e-2 * t8151 * t8159 - 0.37069444444444444444e-2 * t8151 * t8148 - 0.185671721767578125e-4 * t27339 * t29324 - 0.33163888888888888888e-2 * t29355 - 0.23168402777777777778e-3 * t7908 * t29358 - 0.30891203703703703704e-3 * t7908 * t29362 - 0.88437037037037037034e-2 * t29366 - 0.33163888888888888888e-2 * t29370 + 0.24872916666666666666e-2 * t29373 + 0.33163888888888888888e-2 * t28465 - 0.46336805555555555556e-3 * t28467;
    (t29362, t29365, t29366, t29368, t29369, t29370, t29372, t29373, t29377)
}
