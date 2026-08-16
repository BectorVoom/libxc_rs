//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1060/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1060(t14538: f64, t854: f64, t847: f64, t4847: f64, t8684: f64, t849: f64, t3773: f64, t3781: f64, t2487: f64, t4854: f64, t8678: f64, t3789: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14539 = t854 * t14538;
    let t14541 = t847 * t14538;
    let t14550 = t8684 * t4847;
    let t14551 = t14550 * t849;
    let t14553 = t3773 * t3781;
    let t14555 = t2487 * t4854;
    let t14556 = t14555 * t849;
    let t14558 = t8678 * t4847;
    let t14559 = t14558 * t849;
    let t14561 = t3789 * t3781;
    (t14539, t14541, t14551, t14553, t14556, t14559, t14561)
}
