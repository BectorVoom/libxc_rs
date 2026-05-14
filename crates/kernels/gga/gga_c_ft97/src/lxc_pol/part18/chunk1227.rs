//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1227/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1227<F: Float>(t22940: F, t3255: F, t1820: F, t26061: F, t1286: F, t25524: F, t376: F, t25587: F, t5495: F, t26124: F, t11604: F, t12058: F, t1308: F, t1310: F, t2: F, t22873: F, t23133: F, t25523: F, t25577: F, t25609: F, t25610: F, t25615: F, t25616: F, t25847: F, t26: F, t28: F, t2976: F, t4: F, t497: F, t5748: F, t61209: F, t6423: F) -> (F, F, F) {
    let t102350 = t22940 * t3255;
    let t102352 = t26061 * t1820;
    let t102364 = 2.0 / 9.0 * t1286 * t376 * t25524;
    let t102366 = 2.0 / 9.0 * t5495 * t25587;
    let t102369 = 2.0 / 9.0 * t1286 * t376 * t26124;
    let t102370 = 4.0 / 9.0 * t25577 * t25609 * t25610 * t11604 - 4.0 / 27.0 * t25577 * t25615 * t25616 * t11604 - 2.0 * t2976 * t5748 - t23133 * t6423 / 3.0 - 2.0 / 3.0 * t1286 * t28 * t22873 * t25523 + t61209 * t2 * t4 * t26 * t1310 / 6.0 - 4.0 * t102350 - 2.0 * t102352 + t1286 * t28 * t1308 * t12058 / 6.0 + t1286 * t28 * t25847 * t497 / 3.0 + t102364 + t102366 + t102369;
    (t102350, t102352, t102370)
}
