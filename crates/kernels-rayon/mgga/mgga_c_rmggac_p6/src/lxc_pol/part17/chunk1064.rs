//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1064/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1064(t1864: f64, t1971: f64, t209: f64, t236: f64, t36336: f64, t476: f64, t40231: f64, t9222: f64, t36601: f64, t41717: f64, t41723: f64, t41726: f64, t41727: f64, t41736: f64, t41763: f64, t43839: f64, t47439: f64, t47442: f64, t47445: f64, t47450: f64, t47455: f64, t47460: f64, t47465: f64) -> f64 {
    let t47471 = t36336 * t1971 * t236 * t1864 * t476 * t209;
    let t47473 = t9222 * t40231;
    let t47475 = -0.30487649791575028314e-3_f64 * t47439 - 0.72042316457491791906e-3_f64 * t47442 + t41717 - t41723 - t41726 + 0.66671395154821946449e-1_f64 * t41727 + t36601 - t41736 + 0.14967802127329760705e-1_f64 * t47445 - 0.51077519871957407276e-4_f64 * t47450 + t41763 + t43839 + 0.53205749866622299248e-5_f64 * t47455 + 0.1064114997332445985e-4_f64 * t47460 - 0.31923449919973379548e-4_f64 * t47465 - 0.11971293719990017331e-4_f64 * t47471 + 0.1064114997332445985e-4_f64 * t47473;
    t47475
}
