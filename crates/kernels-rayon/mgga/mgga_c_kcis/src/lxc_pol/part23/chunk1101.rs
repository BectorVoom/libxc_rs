//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1101/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1101(t27387: f64, t5644: f64, t1394: f64, t2237: f64, t28426: f64, t1466: f64, t1982: f64, t1490: f64, t303: f64, t1498: f64, t1983: f64, t2002: f64, t27475: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28519 = t27387 * t5644;
    let t28520 = t1394 * t28519;
    let t28522 = t2237 * t28426;
    let t28524 = t1982 * t1466;
    let t28525 = t28524 * t1490;
    let t28526 = t303 * t28525;
    let t28528 = t1983 * t1498;
    let t28529 = t303 * t28528;
    let t28531 = t27475 * t2002;
    (t28519, t28520, t28522, t28524, t28525, t28526, t28528, t28529, t28531)
}
