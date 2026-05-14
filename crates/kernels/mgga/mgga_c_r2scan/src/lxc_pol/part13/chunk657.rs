//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 657/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk657<F: Float>(t5269: F, t5270: F, t5267: F, t234: F, t1743: F, t704: F, t740: F, t717: F, t749: F, t1696: F, t741: F, t225: F, t1836: F, t703: F, t716: F, t224: F) -> (F, F, F, F, F, F, F) {
    let t5271 = t5269 * t5270;
    let t5272 = t5267 * t5271;
    let t5274 = 0.91082604192152556044e5 * t234 * t5272;
    let t5275 = t704 * t1743;
    let t5276 = t5275 * t740;
    let t5278 = 0.35089341735807877242e1 * t234 * t5276;
    let t5279 = t717 * t1743;
    let t5280 = t5279 * t749;
    let t5282 = 0.51947577317044391277e2 * t234 * t5280;
    let t5283 = t1696 * t741;
    let t5285 = t225 * t5270;
    let t5286 = t1836 * t5285;
    let t5288 = 0.14035736694323150897e2 * t234 * t5286;
    let t5290 = 1.0 / t716 / t703;
    let t5291 = t5290 * t224;
    (t5274, t5278, t5282, t5283, t5288, t5290, t5291)
}
