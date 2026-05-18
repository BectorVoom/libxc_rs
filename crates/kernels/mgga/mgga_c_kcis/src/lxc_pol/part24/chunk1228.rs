//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1228/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1228<F: Float>(t19926: F, t7748: F, t19934: F, t19931: F, t92447: F, t5048: F, t95351: F, t5073: F, t95381: F, t1200: F, t18463: F, t100001: F, t100003: F, t100005: F, t100007: F, t100009: F, t99984: F, t99986: F, t99988: F, t99990: F, t99992: F, t99994: F, t99997: F, t99999: F) -> (F, F, F, F, F, F, F) {
    let t100011 = t7748 * t19926;
    let t100013 = t7748 * t19934;
    let t100015 = t92447 * t19931;
    let t100017 = t95351 * t5048;
    let t100019 = t95381 * t5073;
    let t100021 = t18463 * t1200;
    let t100023 = t99984 / F::new(16.0) - t99986 / F::new(8.0) + t99988 / F::new(128.0) + t99990 / F::new(3.0) + t99992 / F::new(9.0) - t99994 / F::new(32.0) - t99997 / F::new(16.0) - t99999 / F::new(36.0) - t100001 / F::new(6.0) - t100003 / F::new(96.0) + t100005 / F::new(36.0) + t100007 / F::new(12.0) + t100009 / F::new(48.0) - t100011 / F::new(12.0) + t100013 / F::new(72.0) - F::new(3.0) / F::new(8.0) * t100015 + t100017 / F::new(4.0) + t100019 / F::new(48.0) - t100021 / F::new(96.0);
    (t100011, t100013, t100015, t100017, t100019, t100021, t100023)
}
