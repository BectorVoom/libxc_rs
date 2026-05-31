//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 289/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk289<F: Float>(t442: F, t451: F, t1413: F, t484: F, t1414: F, t492: F, t1161: F, t512: F, t507: F, t1184: F, t515: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t1472 = t451 * t442;
    let t1486 = t484 * t1413;
    let t1487 = t1486 * sigma0;
    let t1504 = t1414 * t492;
    let t1522 = F::cast_from(0.17123333333333333333e-1_f64) * t1161;
    let t1527 = t512 * t512;
    let t1528 = F::cast_from(1.0_f64) / t1527;
    let t1529 = t507 * t1528;
    let t1531 = F::cast_from(0.516475e0_f64) * t1161;
    let t1534 = F::cast_from(0.104195e0_f64) * t1184;
    let t1537 = F::cast_from(1.0_f64) / t515;
    (t1472, t1486, t1487, t1504, t1522, t1527, t1528, t1529, t1531, t1534, t1537)
}
