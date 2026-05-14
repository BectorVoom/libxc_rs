//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 447/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk447<F: Float>(t53: F, t60: F, t1794: F, t3985: F, t1797: F, t912: F, t3878: F, t814: F, t1395: F, t280: F, t57: F, t815: F, t1802: F, t3998: F, t1805: F, t921: F, t1403: F, t284: F, t62: F, zeta_threshold: F) -> (F, F, F, F) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t5850 = t3985 * t1794;
    let t5855 = t912 * t1797;
    let t5860 = -2.0 * t814 - 6.0 * t3878;
    let t5864 = piecewise3(t54, 0.0, -8.0 / 27.0 * t5850 * t280 + 16.0 / 9.0 * t1395 * t815 + 4.0 / 9.0 * t5855 * t280 + 4.0 / 3.0 * t57 * t5860);
    let t5865 = t3998 * t1802;
    let t5870 = t921 * t1805;
    let t5873 = -t5860;
    let t5877 = piecewise3(t61, 0.0, -8.0 / 27.0 * t5865 * t284 - 16.0 / 9.0 * t1403 * t815 + 4.0 / 9.0 * t5870 * t284 + 4.0 / 3.0 * t62 * t5873);
    (t5860, t5864, t5873, t5877)
}
