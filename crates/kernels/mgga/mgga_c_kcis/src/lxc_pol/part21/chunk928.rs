//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 928/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk928<F: Float>(t15573: F, t5331: F, t1251: F, t10996: F, t11086: F, t15531: F, t15535: F, t15541: F, t15547: F, t15549: F, t15555: F, t15558: F, t15563: F, t15570: F, t1847: F, t3490: F, t3514: F, t5316: F, t5326: F, t5332: F) -> (F,) {
    let t15574 = t15573 * t5331;
    let t15576 = t1251 * t15574 / 144.0;
    let t15577 = -t3514 * t15531 / 432.0 - t3514 * t15535 / 72.0 + t11086 * t5326 / 108.0 + t3514 * t15541 / 288.0 + 11.0 / 648.0 * t10996 * t1847 - t15547 - t15549 / 2592.0 - t3490 * t5316 / 54.0 - 7.0 / 864.0 * t15555 + t1251 * t15558 / 96.0 - t1251 * t15563 / 32.0 - t3490 * t5332 / 18.0 + t1251 * t15570 / 48.0 + t15576;
    (t15577,)
}
