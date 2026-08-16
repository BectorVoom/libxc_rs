//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1286/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1286(t2341: f64, t38: f64, t91: f64, t9384: f64, t2177: f64, t2585: f64, t2281: f64, t8134: f64, t2331: f64, t8130: f64, t656: f64, t8139: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t110093 = t38 * t2341;
    let t110097 = t91 * t9384;
    let t110102 = 154.0_f64 / 27.0_f64 * t2585 * t2177;
    let t110103 = t2281 * t8134;
    let t110140 = t2281 * t2331;
    let t110141 = t110140 * t8130;
    let t110143 = t2281 * t656;
    let t110144 = t110143 * t8139;
    (t110093, t110097, t110102, t110103, t110140, t110141, t110143, t110144)
}
