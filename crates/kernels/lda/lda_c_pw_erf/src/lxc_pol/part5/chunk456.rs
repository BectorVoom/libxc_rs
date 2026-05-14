//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 456/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk456<F: Float>(t668: F, t858: F, t739: F, t92: F, t34: F, t659: F, t743: F, t93: F, t661: F, t108: F, t348: F, t352: F, t462: F, t1409: F, t1412: F, t1420: F, t1424: F, t1429: F, t1435: F, t1436: F, t1439: F, t2009: F, t2013: F, t2016: F, t2020: F, t2025: F, t2029: F, t2033: F, t2037: F, t2039: F, t267: F) -> (F, F, F, F, F) {
    let t2266 = t858 * t668;
    let t2268 = t92 * t739;
    let t2271 = t659 * t34;
    let t2274 = t93 * t743;
    let t2277 = t661 * t34;
    let t2281 = (20.0 / 9.0 * t2268 * t348 + 8.0 / 3.0 * t2271 * t462 + 20.0 / 9.0 * t2274 * t352 - 8.0 / 3.0 * t2277 * t462) * t108;
    let t2287 = t2009 - t2013 - t2016 + t2020 - t2025 + t2029 - t2033 + t2037 - 2.0 / 45.0 * t2266 - t2281 * t267 / 15.0 + t1409 - t1412 + t1420 / 3.0 + 0.06077777777777778 * t1424 + t1429 + t1435 + 2.0 / 9.0 * t1436 + t1439 + t2039;
    (t2266, t2268, t2274, t2281, t2287)
}
