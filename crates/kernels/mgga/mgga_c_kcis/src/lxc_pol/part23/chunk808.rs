//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 808/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk808<F: Float>(t3815: F, t5804: F, t5498: F, t3255: F, t5495: F, t5500: F, t3820: F, t509: F, t3781: F, t5458: F, t1409: F, t1897: F, t3786: F, t518: F, t5526: F, t1319: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16403 = t5804 * t3815;
    let t16404 = t5498 * t16403;
    let t16408 = 0.19711289e-2 * t3255 * t5495;
    let t16410 = 0.26281718666666666666e-2 * t3255 * t5500;
    let t16411 = t509 * t3820;
    let t16412 = t5458 * t3781;
    let t16413 = t16411 * t16412;
    let t16416 = t1409 * t1897;
    let t16417 = t16416 * t3815;
    let t16418 = t3786 * t16417;
    let t16421 = t518 * t5526;
    let t16422 = t16421 * t1319;
    (t16403, t16404, t16408, t16410, t16412, t16413, t16417, t16418, t16422)
}
