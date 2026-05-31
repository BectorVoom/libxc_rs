//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 260/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk260<F: Float>(t411: F, t338: F, t157: F, t301: F, t342: F, t341: F, t69: F) -> (F, F, F, F, F, F) {
    let t1218 = t411 * t411;
    let t1219 = F::cast_from(1.0_f64) / t1218;
    let t1220 = t338 * t1219;
    let t1222 = t342 * t157 * t301;
    let t1223 = F::cast_from(0.17808333333333333333e-1_f64) * t1222;
    let t1224 = t341 * t69;
    (t1218, t1219, t1220, t1222, t1223, t1224)
}
