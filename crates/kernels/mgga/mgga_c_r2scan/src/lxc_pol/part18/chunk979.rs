//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 979/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk979<F: Float>(t39502: F, t10698: F, t11702: F, t10885: F, t11744: F, t2834: F, t3344: F, t1615: F, t3320: F, t783: F, t978: F, t2553: F, t37764: F, t10894: F, t2630: F, t10844: F, t11760: F, t2201: F) -> (F, F, F, F, F, F, F, F) {
    let t39503 = 0.76830240467580968652e0 * t39502;
    let t39511 = t10698 * t11702;
    let t39512 = 0.12805040077930161442e0 * t39511;
    let t39522 = t11744 * t10885;
    let t39523 = 0.23115257973478049502e0 * t39522;
    let t39548 = t2834 * t3344;
    let t39549 = 0.47609969197673950972e-2 * t39548;
    let t39558 = t783 * t978 * t1615 * t3320;
    let t39579 = t37764 * t2553;
    let t39580 = 0.25610080155860322884e0 * t39579;
    let t39601 = t10894 * t2630;
    let t39602 = 0.54878743191129263322e-2 * t39601;
    let t39607 = t2201 * t11760 * t10844;
    (t39503, t39512, t39523, t39549, t39558, t39580, t39602, t39607)
}
