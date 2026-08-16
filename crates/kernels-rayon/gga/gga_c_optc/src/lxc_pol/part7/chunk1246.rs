//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1246/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1246(t24555: f64, t953: f64, t2797: f64, t8182: f64, t25560: f64, t3881: f64, t2606: f64, t7433: f64, t2668: f64, t877: f64, t25029: f64, t2619: f64, t2758: f64, t2760: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25730 = t953 * t24555;
    let t25740 = t2797 * t8182;
    let t25742 = t3881 * t25560;
    let t25749 = t7433 * t2606;
    let t25751 = t2668 * t25749 * t877;
    let t25753 = t953 * t25029;
    let t25769 = t2758 * t2619 * t2760;
    (t25730, t25740, t25742, t25749, t25751, t25753, t25769)
}
