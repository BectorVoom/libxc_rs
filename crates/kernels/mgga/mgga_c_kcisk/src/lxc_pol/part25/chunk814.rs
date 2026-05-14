//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 814/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk814<F: Float>(t11313: F, t1879: F, t3521: F, t4620: F, t4600: F, t4632: F, t1417: F, t4686: F, t4626: F, t4654: F, t1889: F, t3517: F, t1884: F, t10671: F, t677: F, t1821: F, t4663: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11314 = t11313 * t1879;
    let t11316 = t3521 * t4620;
    let t11318 = t3521 * t4600;
    let t11320 = t3521 * t4632;
    let t11338 = t1417 * t4686;
    let t11340 = t1417 * t4626;
    let t11342 = t1417 * t4654;
    let t11344 = t3517 * t1889;
    let t11350 = t3517 * t1884;
    let t11352 = t10671 * t677;
    let t11355 = t4663 * t1821;
    (t11314, t11316, t11318, t11320, t11338, t11340, t11342, t11344, t11350, t11352, t11355)
}
