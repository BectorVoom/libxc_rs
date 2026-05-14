//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1088/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1088<F: Float>(t1196: F, t18463: F, t19882: F, t95391: F, t20155: F, t283: F, t7749: F, t99865: F, t99867: F, t99869: F, t99871: F, t99874: F, t99876: F, t99878: F, t99880: F, t99882: F, t99884: F, t99886: F, t99888: F, t99890: F, t99892: F, t99894: F) -> (F, F, F, F) {
    let t99896 = t18463 * t1196;
    let t99898 = t95391 * t19882;
    let t99900 = t20155 * t283;
    let t99901 = t99900 * t7749;
    let t99903 = t99865 / 27.0 - 2.0 / 9.0 * t99867 + t99869 / 12.0 - t99871 / 64.0 - t99874 / 32.0 + t99876 / 8.0 - t99878 / 128.0 + t99880 / 288.0 - t99882 / 48.0 + t99884 / 96.0 + t99886 / 6.0 + t99888 / 6.0 + t99890 / 144.0 - t99892 / 288.0 - t99894 / 72.0 + t99896 / 24.0 + 3.0 / 64.0 * t99898 - t99901 / 16.0;
    (t99896, t99898, t99901, t99903)
}
