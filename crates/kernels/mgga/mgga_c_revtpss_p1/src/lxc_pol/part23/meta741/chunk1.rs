//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2521/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2521<F: Float>(t10538: F, t51297: F, t213: F, t225: F, t40321: F, t14574: F, t2439: F, t2777: F, t10069: F, t14504: F, t14557: F, t9303: F) -> (F, F, F, F, F) {
    let t51298 = t51297 * t10538;
    let t51299 = F::cast_from(0.34697458558045176417e-2_f64) * t51298;
    let t51320 = t213 * t225 * t40321;
    let t51355 = t2439 * t2777 * t14574;
    let t51373 = t10069 * t14504;
    let t51374 = F::cast_from(0.21951497276451705329e-1_f64) * t51373;
    let t51390 = t9303 * t14557;
    (t51299, t51320, t51355, t51374, t51390)
}
