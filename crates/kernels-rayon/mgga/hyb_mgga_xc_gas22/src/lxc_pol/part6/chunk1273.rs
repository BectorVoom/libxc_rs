//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1273/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1273(t43: f64, t27405: f64, t81: f64, t9999: f64, t3876: f64, t6127: f64, t10013: f64, t10022: f64, t10043: f64, t10046: f64, t10057: f64, t10063: f64, t1954: f64, t1967: f64, t19952: f64, t19960: f64, t23488: f64, t3086: f64, t3087: f64, t3093: f64, t3099: f64, t3881: f64, t3882: f64, t3898: f64, t6088: f64, t8103: f64, t8130: f64) -> (f64, f64, f64) {
    let t45 = 0.135e1_f64 < t43;
    let t27443 = piecewise3(t45, 0.0_f64, t27405);
    let t27474 = t81 * t9999;
    let t27499 = t6127 * t3876;
    let t27530 = -75.0_f64 / 2.0_f64 * t3898 * t8103 + 15.0_f64 / 2.0_f64 * t1954 * t3876 * t8103 + t27499 * t8103 / 8.0_f64 + t3093 * t23488 / 2.0_f64 + t10057 * t6088 / 8.0_f64 + t19960 * t3881 * t8103 / 16.0_f64 - 2.0_f64 * t10063 * t23488 - t8130 * t10022 - 2.0_f64 * t3099 * t27474 + 15.0_f64 / 2.0_f64 * t3882 * t6088 + 85.0_f64 / 4.0_f64 * t10013 * t8103 - 4.0_f64 * t3086 * t23488 - 5.0_f64 / 2.0_f64 * t10043 * t6088 - 19.0_f64 / 8.0_f64 * t19952 * t3881 * t8103 - 4.0_f64 * t1967 * t9999 * t3087 - 2.0_f64 * t10046 * t6088;
    (t27443, t27474, t27530)
}
