//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 968/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk968<F: Float>(t30817: F, t9645: F, t1849: F, t1983: F, t7380: F, t4680: F, t7575: F, t9669: F, t1181: F, t5549: F, t604: F, t5544: F, t1992: F, t30154: F, t7586: F, t30219: F, t9653: F) -> (F, F, F, F, F, F, F, F) {
    let t38939 = t30817 * t9645;
    let t38942 = t7380 * t1983 * t1849;
    let t38946 = t7575 * t4680 * t9669;
    let t38950 = t7575 * t1181 * t604 * t5549;
    let t38954 = t7575 * t1181 * t604 * t5544;
    let t38956 = t1992 * t1849;
    let t38958 = t30154 * t7586 * t38956;
    let t38960 = t30219 * t9653;
    (t38939, t38942, t38946, t38950, t38954, t38956, t38958, t38960)
}
