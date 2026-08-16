//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1350/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1350<F: Float>(t22289: F, t4163: F, t4162: F, t16771: F, t17277: F, t22261: F, t22263: F, t22266: F, t22268: F, t22273: F, t22277: F, t22280: F, t22282: F, t22287: F) -> (F, F) {
    let t22290 = t4163 * t22289;
    let t22291 = t4162 * t22290;
    let t22292 = t16771 * t22291;
    let t22294 = F::cast_from(0.66327777777777777776e-2_f64) * t22261 + t17277 - F::cast_from(0.22109259259259259259e-2_f64) * t22263 - F::cast_from(0.14739506172839506172e-1_f64) * t22266 + F::cast_from(0.22109259259259259258e-2_f64) * t22268 - F::cast_from(0.44218518518518518516e-2_f64) * t22273 + F::cast_from(0.33163888888888888888e-2_f64) * t22277 + F::cast_from(0.88437037037037037033e-2_f64) * t22280 - F::cast_from(0.33163888888888888888e-2_f64) * t22282 - F::cast_from(0.33163888888888888888e-2_f64) * t22287 + F::cast_from(0.66327777777777777776e-2_f64) * t22292;
    (t22292, t22294)
}
