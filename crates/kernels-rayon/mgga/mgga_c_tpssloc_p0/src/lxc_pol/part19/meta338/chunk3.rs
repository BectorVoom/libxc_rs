//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1207/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1207(t10108: f64, t257: f64, t68: f64, t2719: f64, t2627: f64, t2710: f64, t10016: f64, t252: f64, t9957: f64, t852: f64, t9971: f64, t2631: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40889 = 1.0_f64 / t10108 / t257;
    let t40890 = t68 * t40889;
    let t40891 = t2719 * t2719;
    let t40895 = t2627 * t2710;
    let t40904 = t10016 * t68;
    let t40909 = t252 * t9957;
    let t40917 = t9971 * t852;
    let t40925 = t2631 * t2631;
    (t40890, t40891, t40895, t40904, t40909, t40917, t40925)
}
