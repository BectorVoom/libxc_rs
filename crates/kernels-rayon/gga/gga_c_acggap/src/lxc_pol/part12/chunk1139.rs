//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1139/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1139(t1017: f64, t2060: f64, t2288: f64, t36222: f64, t4258: f64, t8806: f64, t30248: f64, t532: f64, t537: f64, t7637: f64, t8859: f64, t1576: f64, t7614: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36225 = t2060 * t36222 * t2288 * t1017;
    let t36227 = t8806 * t4258;
    let t36231 = t30248 * t532;
    let t36236 = t30248 * t537;
    let t36238 = t7637 * t8859;
    let t36240 = t7614 * t1576;
    (t36225, t36227, t36231, t36236, t36238, t36240)
}
