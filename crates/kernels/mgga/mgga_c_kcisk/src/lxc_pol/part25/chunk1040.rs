//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1040/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1040<F: Float>(t18531: F, t1974: F, t16517: F, t1685: F, t16321: F, t16323: F, t16325: F, t16328: F, t16331: F, t16334: F, t16338: F, t16341: F, t16345: F, t1966: F, t1979: F, t5405: F, t7506: F) -> (F,) {
    let t18532 = t18531 * t1974;
    let t18537 = t16517 * t1685;
    let t18540 = 1.0 * t1966 * t18532 - t16321 + t16323 - t16325 + t16328 + t16331 + t16334 - t16338 - t16341 - t16345 + 0.11696446794910408142e1 * t5405 * t7506 + 0.58482233974552040708e0 * t1979 * t18537;
    (t18540,)
}
