//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1063/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1063<F: Float>(t21294: F, t328: F, t6552: F, t331: F, t863: F, t20934: F, t858: F, t867: F, t21287: F, t6240: F, t21033: F, t866: F, t19553: F, t21269: F, t21274: F, t21280: F, t21286: F, t2343: F, t2345: F, t6220: F, t6308: F, t6555: F, t904: F, t916: F, t929: F, t933: F) -> (F, F, F, F, F) {
    let t21295 = 35.0 / 36.0 * t21294;
    let t21296 = t6552 * t328;
    let t21298 = t863 * t21296 * t331;
    let t21302 = t21298 * t867 * t858 * t20934 / 4.0;
    let t21306 = 3.0 / 8.0 * t6240 * t867 * t858 * t21287;
    let t21310 = t866 * t867 * t858 * t21033 / 96.0;
    let t21311 = 595.0 / 576.0 * t21269 - t21274 - t929 * t933 * t904 * t19553 / 768.0 + t21280 + t2343 * t2345 * t6308 * t6220 / 64.0 - t21286 - 3.0 / 64.0 * t6555 * t916 * t904 * t21287 + t21295 + t21302 - t21306 - t21310;
    (t21295, t21302, t21306, t21310, t21311)
}
