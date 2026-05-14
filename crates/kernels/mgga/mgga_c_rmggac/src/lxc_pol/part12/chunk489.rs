//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 489/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk489<F: Float>(t221: F, t446: F, t5605: F, t1475: F, t998: F, t1494: F, t476: F, t209: F, t1184: F, t1212: F, t1468: F, t1515: F, t1516: F, t1228: F, t1518: F, t1190: F, t1497: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5607 = t221 * t5605 * t446;
    let t5611 = t221 * t1475 * t998;
    let t5614 = t1494 * t476;
    let t5615 = t5614 * t209;
    let t5616 = t221 * t5615;
    let t5619 = t1494 * t1184;
    let t5620 = t5619 * t476;
    let t5621 = t221 * t5620;
    let t5624 = t1468 * t1212;
    let t5625 = t221 * t5624;
    let t5630 = t1515 * t1516 * t998;
    let t5633 = t1228 * t1518;
    let t5636 = 0.12805126321218922714e0 * t1190 * t1497;
    (t5607, t5611, t5615, t5616, t5620, t5621, t5624, t5625, t5630, t5633, t5636)
}
