//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 674/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk674<F: Float>(t1885: F, t5394: F, t587: F, t1697: F, t212: F, t22: F, t219: F, t5063: F, t4367: F, t639: F, t1774: F, t586: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5395 = t1885 * t5394;
    let t5397 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t587 * t5395;
    let t5399 = F::cast_from(1.0_f64) / t212 / t1697;
    let t5400 = t22 * t5399;
    let t5401 = t219 * t5063;
    let t5402 = t5401 * t4367;
    let t5403 = t5400 * t5402;
    let t5405 = F::cast_from(32.0_f64) / F::cast_from(81.0_f64) * t639 * t5403;
    let t5406 = t1774 * t586;
    (t5395, t5397, t5399, t5400, t5401, t5402, t5403, t5405, t5406)
}
