//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1027/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1027<F: Float>(t1852: F, t3886: F, t1859: F, t3864: F, t3031: F, t544: F, t3: F, t3032: F, t3890: F, t3854: F, t3025: F, t1867: F, t10273: F, t574: F, t577: F, t10320: F, t10324: F, t10329: F, t10333: F, t3040: F, t571: F, t6132: F, t6135: F, t8216: F, t8219: F, t8221: F, t8226: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10336 = t1852 * t3886;
    let t10338 = t1859 * t3864;
    let t10340 = t3031 * t10338 * t544;
    let t10344 = t3031 * t3032 * t3;
    let t10347 = t1852 * t3890;
    let t10349 = t1859 * t3854;
    let t10351 = t3025 * t10349 * t544;
    let t10354 = t1867 * t3854;
    let t10356 = t3031 * t10354 * t544;
    let t10360 = t574 * t577 * t10273;
    let t10363 = -t6132 - 2.0 / 243.0 * t6135 - 4.0 / 243.0 * t8216 + t8219 - t8221 + 2.0 / 81.0 * t8226 + t10320 / 243.0 - 5.0 / 243.0 * t571 * t10324 + 2.0 / 27.0 * t571 * t10329 - 4.0 / 81.0 * t3040 * t10333 - t10336 / 81.0 - t571 * t10340 / 9.0 + 4.0 / 27.0 * t3040 * t10344 + t10347 / 162.0 - t571 * t10351 / 81.0 + t571 * t10356 / 27.0 - t571 * t10360 / 54.0;
    (t10338, t10340, t10344, t10349, t10351, t10354, t10356, t10360, t10363)
}
