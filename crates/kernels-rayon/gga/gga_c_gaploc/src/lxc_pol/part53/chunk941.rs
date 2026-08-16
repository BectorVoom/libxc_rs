//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 941/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk941(t10057: f64, t13045: f64, t13013: f64, t5782: f64, t1445: f64, t2087: f64, t3234: f64, t8483: f64, t3009: f64, t9688: f64, t41512: f64, t41515: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44164 = 0.25025342966295298669e1_f64 * t10057 * t13045;
    let t44170 = 0.69017266717057349418e1_f64 * t5782 * t13013;
    let t44174 = 0.69017266717057349418e1_f64 * t2087 * t1445 * t8483 * t3234;
    let t44178 = 0.69017266717057349418e1_f64 * t2087 * t1445 * t3009 * t9688;
    let t44179 = 0.17875244975925213335e0_f64 * t41512;
    let t44180 = 0.29792074959875355558e-1_f64 * t41515;
    (t44164, t44170, t44174, t44178, t44179, t44180)
}
