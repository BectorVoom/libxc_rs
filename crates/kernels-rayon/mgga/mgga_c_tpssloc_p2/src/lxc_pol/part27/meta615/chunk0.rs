//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2091/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2091(t23460: f64, t995: f64, t23452: f64, t6739: f64, t6741: f64, t23482: f64, t23488: f64, t23508: f64, t6721: f64, t1937: f64, t23453: f64, t40: f64) -> (f64, f64, f64, f64, f64) {
    let t83098 = t23460 * t995;
    let t83111 = t23452 * t6739 * t6741;
    let t83114 = t23482 * t23488;
    let t83120 = t6721 * t23508;
    let t83127 = t23453 * t40 * t1937;
    (t83098, t83111, t83114, t83120, t83127)
}
