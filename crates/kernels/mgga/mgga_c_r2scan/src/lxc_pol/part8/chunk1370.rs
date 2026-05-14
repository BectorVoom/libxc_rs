//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1370/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1370<F: Float>(t21326: F, t21330: F, t21333: F, t21340: F, t21344: F, t21345: F, t21349: F, t21350: F, t21354: F, t21357: F, t21361: F, t28579: F, t28582: F, t21365: F, t21370: F, t21371: F, t21375: F, t21379: F, t21383: F, t21387: F, t21392: F, t21394: F, t26476: F, t28592: F, t28595: F, t28598: F, t28601: F) -> (F, F) {
    let t33483 = -0.1200612870296e-1 * t28579 - 0.1200612870296e-1 * t28582 + 0.28895839882605942646e1 * t21326 + t21330 + t21333 + t21340 - t21344 + 0.32530743900905219526e-1 * t21345 + t21349 - 60.0 * t21350 - t21354 + t21357 + t21361;
    let t33490 = t21365 + t21370 - 0.16936279733333333332e-2 * t21371 + t26476 + t21375 + t21379 + t21383 - 0.60030643514799999999e-2 * t28592 - 0.60030643514799999999e-2 * t28595 - t21387 - 0.1714584e0 * t28598 - 0.1714584e0 * t28601 + t21392 + 0.1524265176e-1 * t21394;
    (t33483, t33490)
}
