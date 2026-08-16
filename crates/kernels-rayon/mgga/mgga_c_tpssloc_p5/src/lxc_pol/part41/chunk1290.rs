//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1290/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1290(t2331: f64, t2585: f64, t1851: f64, t8217: f64, t110075: f64, t30281: f64, t29895: f64, t30285: f64, t30304: f64, t29900: f64, t30308: f64, t110140: f64, t8262: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t110601 = t2585 * t2331;
    let t110919 = 2.0_f64 * t1851 * t8217;
    let t111056 = 4.0_f64 * t110075 * t30281;
    let t111058 = 20.0_f64 / 9.0_f64 * t29895 * t30285;
    let t111077 = 20.0_f64 / 9.0_f64 * t29895 * t30304;
    let t111079 = 20.0_f64 / 27.0_f64 * t29900 * t30308;
    let t111101 = t110140 * t8262;
    (t110601, t110919, t111056, t111058, t111077, t111079, t111101)
}
