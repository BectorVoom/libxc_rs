//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 557/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk557<F: Float>(t719: F, t4808: F, t1990: F, t1993: F, t4636: F, t1961: F, t1965: F, t1964: F, t760: F, t755: F, t4722: F, t763: F, t1670: F, t1676: F, t4761: F, t591: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5330 = 1.0 / t719;
    let t5344 = 0.38691203703703703703e-3 * t4808;
    let t5348 = t1990 * t1993;
    let t5360 = 0.22831111111111111111e-1 * t4636;
    let t5368 = t1961 * t1965;
    let t5371 = t1964 * t760;
    let t5372 = 1.0 / t5371;
    let t5373 = t755 * t5372;
    let t5380 = 0.68863333333333333333e0 * t4636;
    let t5387 = 0.17365833333333333333e0 * t4722;
    let t5396 = t1964 * t1964;
    let t5397 = 1.0 / t5396;
    let t5398 = t755 * t5397;
    let t5399 = t763 * t763;
    let t5400 = 1.0 / t5399;
    let t5405 = t1670 * t1676;
    let t5408 = t591 * t4761;
    (t5330, t5344, t5348, t5360, t5368, t5372, t5373, t5380, t5387, t5396, t5397, t5398, t5399, t5400, t5405, t5408)
}
