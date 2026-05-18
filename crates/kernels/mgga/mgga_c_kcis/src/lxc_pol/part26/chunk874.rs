//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 874/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk874<F: Float>(t1495: F, t20956: F, t1468: F, t1464: F, t7321: F, t1397: F, t1394: F, t5672: F, t5748: F, t5752: F, t5876: F, t1489: F, t6281: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t20957 = t1495 * t20956;
    let t20958 = t1468 * t20957;
    let t20959 = t1464 * t20958;
    let t20961 = t7321 * sigma2;
    let t20962 = t20961 * t1397;
    let t20963 = t1394 * t20962;
    let t20965 = t5748 * t5672;
    let t20966 = t1464 * t20965;
    let t20969 = t5752 * t5876;
    let t20970 = t1464 * t20969;
    let t20974 = t6281 * t1489;
    (t20959, t20961, t20963, t20966, t20970, t20974)
}
