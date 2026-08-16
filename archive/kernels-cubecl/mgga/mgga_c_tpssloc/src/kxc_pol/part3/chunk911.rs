//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 911/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk911<F: Float>(t340: F, t63: F, t344: F, t221: F, t339: F, t2960: F, t2974: F, t135: F, t3016: F, t973: F, t1036: F, t3078: F) -> (F, F, F, F, F) {
    let t10335 = t63 * t340;
    let t10336 = t10335 * t344;
    let t10337 = t221 * t10336;
    let t10339 = F::cast_from(0.3086419753086419753e-3_f64) * t339 * t10337;
    let t10342 = t2960 * t2974;
    let t10352 = t135 * t3016;
    let t10353 = t973 * t10352;
    let t10370 = t3078 * t1036;
    (t10335, t10339, t10342, t10353, t10370)
}
