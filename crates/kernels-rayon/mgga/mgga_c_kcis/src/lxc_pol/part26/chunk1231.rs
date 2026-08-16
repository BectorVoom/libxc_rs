//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1231/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1231(t1494: f64, t94424: f64, t1458: f64, t4121: f64, t11881: f64, t7925: f64, t2237: f64, t54162: f64, t7915: f64, t7900: f64, t11425: f64, t1386: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94425 = t94424 * t1494;
    let t94453 = t1458 * t4121;
    let t94472 = t11881 * t7925;
    let t94489 = t2237 * t54162 * t7915;
    let t94491 = t54162 * t7900;
    let t94492 = t2237 * t94491;
    let t94519 = t1386 * t11425;
    (t94425, t94453, t94472, t94489, t94491, t94492, t94519)
}
