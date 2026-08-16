//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 792/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk792(t107: f64, t2536: f64, t2021: f64, t1858: f64, t2652: f64, t787: f64, t4820: f64, t7069: f64, t4598: f64, t965: f64, t4585: f64, t948: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7526 = t2536 * t107;
    let t7527 = t2021 * t7526;
    let t7530 = t1858 * t2652;
    let t7531 = t787 * t7530;
    let t7534 = t4820 * t7069;
    let t7539 = t4598 * t965;
    let t7542 = t4585 * t948;
    (t7527, t7530, t7531, t7534, t7539, t7542)
}
