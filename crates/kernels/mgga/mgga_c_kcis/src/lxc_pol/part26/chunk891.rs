//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 891/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk891<F: Float>(t22279: F, t4160: F, t11862: F, t7101: F, t1650: F, t5627: F, t4163: F, t4162: F, t167: F, t2001: F, t16771: F, t17277: F, t22261: F, t22263: F, t22266: F, t22268: F, t22273: F, t22277: F) -> (F, F, F, F, F, F, F) {
    let t22280 = t4160 * t22279;
    let t22282 = t11862 * t7101;
    let t22284 = t1650 * t5627;
    let t22285 = t4163 * t22284;
    let t22286 = t4162 * t22285;
    let t22287 = t4160 * t22286;
    let t22289 = t167 * t2001;
    let t22290 = t4163 * t22289;
    let t22291 = t4162 * t22290;
    let t22292 = t16771 * t22291;
    let t22294 = 0.66327777777777777776e-2 * t22261 + t17277 - 0.22109259259259259259e-2 * t22263 - 0.14739506172839506172e-1 * t22266 + 0.22109259259259259258e-2 * t22268 - 0.44218518518518518516e-2 * t22273 + 0.33163888888888888888e-2 * t22277 + 0.88437037037037037033e-2 * t22280 - 0.33163888888888888888e-2 * t22282 - 0.33163888888888888888e-2 * t22287 + 0.66327777777777777776e-2 * t22292;
    (t22280, t22282, t22285, t22287, t22290, t22292, t22294)
}
