//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1508/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1508(t5168: f64, t592: f64, t5166: f64, t588: f64, t5187: f64, t571: f64, t11981: f64, t2528: f64, t5154: f64, t172: f64, t5151: f64, t763: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15877 = t592 * t5168;
    let t15880 = 8.0_f64 * t588 * t5166;
    let t15883 = t571 * t5187;
    let t15889 = 32.0_f64 * t11981;
    let t15890 = t5154 * t2528;
    let t15892 = t5151 * t172;
    let t15894 = 0.11696447245269292414e1_f64 * t15892 * t763;
    (t15877, t15880, t15883, t15889, t15890, t15892, t15894)
}
