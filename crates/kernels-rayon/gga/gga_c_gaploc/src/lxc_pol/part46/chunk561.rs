//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 561/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk561(t701: f64, t9729: f64, t1445: f64, t2194: f64, t3308: f64, t9734: f64, t2530: f64, t2571: f64, t9604: f64, t9591: f64, t7068: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9869 = t9729 * t701;
    let t9870 = t1445 * t9869;
    let t9873 = t2194 * t3308;
    let t9875 = t9734 * t701;
    let t9876 = t1445 * t9875;
    let t9879 = t2571 * t2530;
    let t9880 = t1445 * t9879;
    let t9883 = t1445 * t9604;
    let t9886 = t1445 * t9591;
    let t9889 = t883 * t7068;
    (t9870, t9873, t9876, t9880, t9883, t9886, t9889)
}
