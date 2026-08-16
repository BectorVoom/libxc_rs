//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 977/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk977(t107: f64, t240: f64, t625: f64, t656: f64, t2331: f64, t63: f64, t192: f64, t532: f64, t1982: f64, t1887: f64, t6916: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22468 = t240 * t107;
    let t22470 = t625 * t656;
    let t22473 = t63 * t2331;
    let t22573 = t192 * t532;
    let t22574 = t1982 * t22573;
    let t22633 = t6916 * t1887;
    (t22468, t22470, t22473, t22573, t22574, t22633)
}
