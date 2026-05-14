//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 703/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk703<F: Float>(t2197: F, t7772: F, t7786: F, t7788: F, t7801: F, t8049: F, t8052: F, t8055: F, t8058: F, t8083: F, t8087: F, t8091: F, t8095: F, t1872: F, t2205: F) -> (F, F) {
    let t8104 = -0.34752604166666666667e-3 * t8083 * t2197 + 0.46377350260416666667e-4 * t7772 * t8087 - t7786 - 0.11584201388888888889e-3 * t7788 * t8091 + 0.34752604166666666667e-3 * t7788 * t8095 + 0.34752604166666666667e-3 * t7788 * t8087 + t7801 + 0.11607361111111111111e-2 * t8049 + 0.17411041666666666666e-2 * t8052 - 0.17411041666666666666e-2 * t8055 + 0.11607361111111111111e-2 * t8058;
    let t8108 = t2205 * t1872;
    (t8104, t8108)
}
