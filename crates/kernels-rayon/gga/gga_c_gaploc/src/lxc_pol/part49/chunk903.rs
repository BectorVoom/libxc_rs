//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 903/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk903(t41609: f64, t10469: f64, t2482: f64, t9267: f64, t2476: f64, t26922: f64, t9438: f64, t10268: f64, t4391: f64, t549: f64, t2365: f64, t31748: f64) -> (f64, f64, f64, f64, f64) {
    let t41610 = 0.15337170381568299871e1_f64 * t41609;
    let t41612 = t9267 * t10469 * t2482;
    let t41613 = 0.72851559312449424385e1_f64 * t41612;
    let t41615 = t2476 * t9438 * t26922;
    let t41616 = 0.15976219147466979032e-1_f64 * t41615;
    let t41618 = t4391 * t549 * t10268;
    let t41619 = 0.11916829983950142223e0_f64 * t41618;
    let t41621 = t4391 * t2365 * t31748;
    (t41610, t41613, t41616, t41619, t41621)
}
