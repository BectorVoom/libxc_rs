//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 808/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk808(t1901: f64, t5737: f64, t5776: f64, t5519: f64, t5557: f64, t5513: f64, t5516: f64, t5522: f64, t5525: f64, t5539: f64, t5541: f64, t5548: f64, t5551: f64, t5553: f64, t5560: f64, t5563: f64, t5566: f64, t5570: f64, t5574: f64) -> (f64, f64, f64, f64, f64) {
    let t5777 = t5737 * t1901;
    let t5779 = 0.96491876992155210402e2_f64 * t5776 * t5777;
    let t5783 = 0.93011851851851851854e0_f64 * t5519;
    let t5790 = 0.36514074074074074075e0_f64 * t5557;
    let t5796 = 0.142419375e1_f64 * t5513 - 0.28483875e1_f64 * t5516 + 0.1898925e1_f64 * t5541 - t5783 + 0.11958666666666666667e1_f64 * t5522 - 0.89690000000000000001e0_f64 * t5525 + 0.8969e0_f64 * t5539 - 0.76790625e-1_f64 * t5548 + 0.46074375e0_f64 * t5551 + 0.3071625e0_f64 * t5553 - t5790 + 0.82156666666666666666e0_f64 * t5560 - 0.49293999999999999999e0_f64 * t5563 - 0.49293999999999999999e0_f64 * t5566 + 0.73941e0_f64 * t5570 + 0.24647e0_f64 * t5574;
    (t5777, t5779, t5783, t5790, t5796)
}
