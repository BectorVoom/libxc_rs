//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 552/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk552(t787: f64, t9816: f64, t2021: f64, t2672: f64, t7372: f64, t7634: f64, t2558: f64, t9286: f64) -> (f64, f64, f64, f64) {
    let t9817 = t787 * t9816;
    let t9820 = t2021 * t2672;
    let t9822 = 0.29792074959875355558e-1_f64 * t9820 * t7372;
    let t9823 = t787 * t7634;
    let t9824 = t9286 * t2558;
    (t9817, t9822, t9823, t9824)
}
