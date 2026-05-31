//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 185/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk185<F: Float>(t706: F, t709: F, t604: F, t456: F) -> (F, F, F) {
    let t710 = t706 * t709;
    let t713 = t604 * t604;
    let t715 = F::cast_from(0.98556445e-3_f64) * t456 * t710 - F::cast_from(2.0_f64) * t713;
    let t716 = F::cast_from(1.0_f64) / t715;
    (t710, t715, t716)
}
