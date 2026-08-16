//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1330/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1330(t14734: f64, t2996: f64, t2998: f64, t3000: f64, t5040: f64, t5043: f64, t5045: f64, t5425: f64, t6032: f64, t6034: f64, t6619: f64, t6622: f64, t6625: f64) -> f64 {
    let t24708 = t14734 + 64.0_f64 * t2996 + 120.0_f64 * t2998 - 16.0_f64 * t3000 - 0.10389515463408878255e3_f64 * t5040 - 0.46785788981077169656e1_f64 * t5043 - 0.35089341735807877242e1_f64 * t5045 + 8.0_f64 * t6032 - 8.0_f64 * t6034 + 12.0_f64 * t6619 + 24.0_f64 * t5425 + 12.0_f64 * t6622 - 4.0_f64 * t6625;
    t24708
}
