//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1284/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1284(t6337: f64, t8012: f64, t898: f64, t1208: f64, t18520: f64, t6283: f64, t2328: f64, t8289: f64, t8293: f64, t6324: f64, t8287: f64, t3135: f64, t6230: f64, t8288: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22511 = 0.51947577317044391277e2_f64 * t898 * t8012 * t6337;
    let t22515 = 0.12304822629859687989e5_f64 * t898 * t18520 * t1208 * t6283;
    let t22517 = 0.30762056574649219973e4_f64 * t2328 * t8289;
    let t22519 = 0.51947577317044391277e2_f64 * t2328 * t8293;
    let t22522 = 0.6233709278045326953e3_f64 * t898 * t8287 * t6324;
    let t22526 = 0.30762056574649219973e4_f64 * t898 * t6230 * t3135 * t8288;
    (t22511, t22515, t22517, t22519, t22522, t22526)
}
