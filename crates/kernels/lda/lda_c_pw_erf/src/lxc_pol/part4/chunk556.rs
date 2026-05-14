//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 556/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk556<F: Float>(t199: F, t2443: F, t108: F, t2325: F, t2329: F, t2334: F, t2337: F, t659: F, t661: F, t92: F, t93: F, t1219: F, t2257: F, t2261: F, t2387: F, t2391: F, t2395: F, t2399: F, t2404: F, t2409: F, t2427: F, t267: F) -> (F, F, F) {
    let t2445 = 2.0 / 15.0 * t2443 * t199;
    let t2455 = (20.0 / 9.0 * t92 * t2325 + 4.0 / 3.0 * t659 * t2329 + 20.0 / 9.0 * t93 * t2334 + 4.0 / 3.0 * t661 * t2337) * t108;
    let t2460 = t1219 + t2387 - t2391 + t2395 - t2399 + t2404 + t2409 + t2427 + t2445 - t2455 * t267 / 15.0 + 2.0 / 3.0 * t2257 + 0.12155555555555556 * t2261;
    (t2445, t2455, t2460)
}
