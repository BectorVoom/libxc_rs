//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1300/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1300(t10525: f64, t10526: f64, t34246: f64, t8063: f64, t9285: f64, t2877: f64, t30642: f64, t30789: f64, t30703: f64, t10597: f64, t1537: f64, t30297: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34249 = 0.21450293971110256001e1_f64 * t10525 * t10526 * t34246;
    let t34251 = 0.47667319935800568892e0_f64 * t9285 * t8063;
    let t34253 = 0.71500979903700853338e0_f64 * t30642 * t2877;
    let t34256 = 0.35750489951850426669e0_f64 * t30789 * t2877;
    let t34258 = 0.71500979903700853338e0_f64 * t30703 * t2877;
    let t34259 = t1537 * t10597;
    let t34260 = 0.25561950635947166451e1_f64 * t34259;
    let t34261 = 0.15976219147466979032e-1_f64 * t30297;
    (t34249, t34251, t34253, t34256, t34258, t34260, t34261)
}
