//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1302/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1302<F: Float>(t10525: F, t10526: F, t34246: F, t8063: F, t9285: F, t2877: F, t30642: F, t30789: F, t30703: F, t10597: F, t1537: F, t30297: F) -> (F, F, F, F, F, F, F) {
    let t34249 = F::cast_from(0.21450293971110256001e1_f64) * t10525 * t10526 * t34246;
    let t34251 = F::cast_from(0.47667319935800568892e0_f64) * t9285 * t8063;
    let t34253 = F::cast_from(0.71500979903700853338e0_f64) * t30642 * t2877;
    let t34256 = F::cast_from(0.35750489951850426669e0_f64) * t30789 * t2877;
    let t34258 = F::cast_from(0.71500979903700853338e0_f64) * t30703 * t2877;
    let t34259 = t1537 * t10597;
    let t34260 = F::cast_from(0.25561950635947166451e1_f64) * t34259;
    let t34261 = F::cast_from(0.15976219147466979032e-1_f64) * t30297;
    (t34249, t34251, t34253, t34256, t34258, t34260, t34261)
}
