//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1064/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1064<F: Float>(t1864: F, t1971: F, t209: F, t236: F, t36336: F, t476: F, t40231: F, t9222: F, t36601: F, t41717: F, t41723: F, t41726: F, t41727: F, t41736: F, t41763: F, t43839: F, t47439: F, t47442: F, t47445: F, t47450: F, t47455: F, t47460: F, t47465: F) -> F {
    let t47471 = t36336 * t1971 * t236 * t1864 * t476 * t209;
    let t47473 = t9222 * t40231;
    let t47475 = -F::cast_from(0.30487649791575028314e-3_f64) * t47439 - F::cast_from(0.72042316457491791906e-3_f64) * t47442 + t41717 - t41723 - t41726 + F::cast_from(0.66671395154821946449e-1_f64) * t41727 + t36601 - t41736 + F::cast_from(0.14967802127329760705e-1_f64) * t47445 - F::cast_from(0.51077519871957407276e-4_f64) * t47450 + t41763 + t43839 + F::cast_from(0.53205749866622299248e-5_f64) * t47455 + F::cast_from(0.1064114997332445985e-4_f64) * t47460 - F::cast_from(0.31923449919973379548e-4_f64) * t47465 - F::cast_from(0.11971293719990017331e-4_f64) * t47471 + F::cast_from(0.1064114997332445985e-4_f64) * t47473;
    t47475
}
