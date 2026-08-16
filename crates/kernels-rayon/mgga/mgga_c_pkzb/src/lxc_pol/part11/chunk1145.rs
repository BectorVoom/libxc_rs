//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1145/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1145(t3652: f64, t5939: f64, t757: f64, t2887: f64, t68: f64, t9554: f64, t9297: f64, t9301: f64, t5931: f64, t9685: f64, t751: f64, t9633: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26535 = t757 * t5939 * t3652;
    let t26585 = t2887 * t68 * t9554;
    let t26588 = t2887 * t68 * t9297;
    let t26592 = t2887 * t68 * t9301;
    let t26646 = t5931 * t9685;
    let t26653 = t751 * t9633;
    (t26535, t26585, t26588, t26592, t26646, t26653)
}
