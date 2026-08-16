//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2917/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2917(t11468: f64, t15541: f64, t981: f64, t11591: f64, t4734: f64, t11602: f64, t4719: f64, t15543: f64, t3022: f64, t15547: f64, t3034: f64, t11610: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52910 = 0.6233709278045326953e3_f64 * t981 * t15541 * t11468;
    let t52912 = 0.51947577317044391277e2_f64 * t11591 * t4734;
    let t52914 = 0.35089341735807877242e1_f64 * t4719 * t11602;
    let t52916 = 0.30762056574649219973e4_f64 * t3022 * t15543;
    let t52918 = 0.51947577317044391276e2_f64 * t15547 * t3034;
    let t52920 = 0.5848223622634646207e0_f64 * t4719 * t11610;
    (t52910, t52912, t52914, t52916, t52918, t52920)
}
