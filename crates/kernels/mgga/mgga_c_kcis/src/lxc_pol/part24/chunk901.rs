//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 901/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk901<F: Float>(t20749: F, t20769: F, t20789: F, t20809: F, t11223: F, t11230: F, t1282: F, t1291: F, t15109: F, t15692: F, t1872: F, t20709: F, t20711: F, t20721: F, t20724: F, t20728: F, t3664: F, t3669: F, t437: F, t5360: F, t5363: F, t5394: F, t6860: F, t6879: F) -> (F, F) {
    let t20811 = t20749 + t20769 + t20789 + t20809;
    let t20813 = 2.0 * t11223 * t6860 - 6.0 * t11230 * t20721 - t1282 * t20811 - t1291 * t20711 - 2.0 * t15109 * t1872 + 4.0 * t15692 * t5363 + t20709 * t437 + 4.0 * t20724 * t3669 + 2.0 * t20728 * t3669 - t3664 * t6879 - 2.0 * t5360 * t5394;
    (t20811, t20813)
}
