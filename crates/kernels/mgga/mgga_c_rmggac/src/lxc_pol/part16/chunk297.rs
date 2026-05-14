//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 297/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk297<F: Float>(t1845: F, t1846: F, t1157: F, t1442: F, t1835: F, t1839: F, t198: F, t454: F, t589: F, t201: F, t597: F, t615: F) -> (F, F, F, F, F, F) {
    let t1847 = t1845 + t1846;
    let t1856 = -0.32163648644302209643e2 * t1847 * t198 + 0.19298189186581325786e3 * t1442 * t589 - 0.38596378373162651572e3 * t1157 * t1839 + 0.96490945932906628929e2 * t454 * t1835;
    let t1857 = t1856 * t201;
    let t1859 = t597 * t597;
    let t1860 = t1859 * t201;
    let t1864 = t615 * t615;
    (t1847, t1856, t1857, t1859, t1860, t1864)
}
