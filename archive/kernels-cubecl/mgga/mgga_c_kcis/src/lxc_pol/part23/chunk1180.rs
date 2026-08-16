//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1180/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1180<F: Float>(t180: F, t2165: F, t228: F, t26425: F, t26561: F, t2772: F, t36429: F, t36439: F, t7657: F, t7669: F, t9010: F, t9017: F, t9018: F, t91791: F, t91793: F, t9185: F, t91863: F, t91866: F, t91869: F, t91872: F, t91874: F, t91902: F, t91905: F, t91963: F, t92019: F, t92064: F, t92104: F, t92158: F, t92165: F, t92168: F, t92170: F, t92339: F, t92344: F, t92376: F) -> F {
    let t92379 = t180 * (t91791 + t91793 + t91863 + F::cast_from(6.0_f64) * t91902 * t2772 - t91866 + t91869 - t91872 + t91874 - F::cast_from(6.0_f64) * t91905 * t9018 + (t91963 + t92019 + t92064 + t92104) * t228 + t92158 - F::cast_from(18.0_f64) * t9017 * t7669 * t2772 - t36429 * t2165 - F::cast_from(18.0_f64) * t36439 * t26425 + F::cast_from(12.0_f64) * t9010 * t26561 - t7657 * t9185 + t92165 - t92168 - t92170 - t92339 - t92344 + t92376);
    t92379
}
