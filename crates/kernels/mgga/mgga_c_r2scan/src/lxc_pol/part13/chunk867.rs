//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 867/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk867<F: Float>(t1266: F, t260: F, t259: F, t277: F, t254: F, t3332: F, t6402: F, t2147: F, t3316: F, t776: F, t2228: F, t57: F, t2116: F, t3320: F, t560: F, t2201: F, t3319: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10831 = t260 * t1266;
    let t10833 = t259 * t10831 * t277;
    let t10834 = t254 * t10833;
    let t10835 = 0.42377972951376424087e0 * t10834;
    let t10836 = t3332 * t6402;
    let t10837 = t2147 * t10836;
    let t10839 = t776 * t3316;
    let t10840 = 0.23115257973478049502e0 * t10839;
    let t10841 = t2228 * t57;
    let t10842 = t10841 * t2116;
    let t10843 = 0.16463622957338778997e-1 * t10842;
    let t10844 = t3320 * t560;
    let t10846 = t2201 * t3319 * t10844;
    (t10831, t10833, t10835, t10836, t10837, t10839, t10840, t10841, t10843, t10844, t10846)
}
