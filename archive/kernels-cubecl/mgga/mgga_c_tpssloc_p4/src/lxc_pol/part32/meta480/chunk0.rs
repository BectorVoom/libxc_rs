//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1781/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1781<F: Float>(t25084: F, t4184: F, t23146: F, t4191: F, t4240: F, t4250: F, t13228: F, t828: F, t2628: F, t6605: F, t13351: F, t232: F) -> (F, F, F, F, F, F, F, F) {
    let t25085 = t25084 * t4184;
    let t25087 = t23146 * t4191;
    let t25089 = t23146 * t4240;
    let t25091 = t23146 * t4250;
    let t25093 = t13228 * t828;
    let t25094 = t2628 * t25093;
    let t25095 = t6605 * t25094;
    let t25097 = t13351 * t232;
    (t25085, t25087, t25089, t25091, t25093, t25094, t25095, t25097)
}
