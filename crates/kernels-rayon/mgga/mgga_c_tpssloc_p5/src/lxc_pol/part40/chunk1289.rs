//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1289/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1289(t110075: f64, t30149: f64, t29895: f64, t30156: f64, t30165: f64, t2331: f64, t2585: f64, t29900: f64, t30172: f64, t111: f64, t8240: f64, t112: f64, t30217: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t110564 = 4.0_f64 * t110075 * t30149;
    let t110566 = 20.0_f64 / 9.0_f64 * t29895 * t30156;
    let t110586 = 20.0_f64 / 9.0_f64 * t29895 * t30165;
    let t110601 = t2585 * t2331;
    let t110615 = 20.0_f64 / 27.0_f64 * t29900 * t30172;
    let t110631 = t8240 * t111;
    let t110684 = t30217 * t112;
    (t110564, t110566, t110586, t110601, t110615, t110631, t110684)
}
