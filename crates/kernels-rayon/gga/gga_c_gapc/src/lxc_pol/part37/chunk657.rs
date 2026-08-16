//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 657/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk657(t3909: f64, t576: f64, t3763: f64, t3776: f64, t3793: f64, t3835: f64, t3836: f64, t3838: f64, t3839: f64, t3840: f64, t3842: f64, t3843: f64, t3844: f64) -> (f64, f64) {
    let t3910 = t576 * t3909;
    let t3914 = t3835 - t3836 - 0.12650553385416666667e-5_f64 * t3763 + t3838 - t3839 - t3840 + 0.57970906942607043475e-5_f64 * t3776 - t3842 + t3843 + t3844 - 0.90579542097823505428e-7_f64 * t3793;
    (t3910, t3914)
}
