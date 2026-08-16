//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 866/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk866(t12357: f64, t381: f64, t452: f64, t1258: f64, t980: f64, t12235: f64, t3036: f64, t2937: f64, t929: f64, t1160: f64, t3065: f64, t930: f64) -> (f64, f64, f64, f64, f64) {
    let t12360 = 0.65854491829355115987e0_f64 * t381 * t452 * t12357;
    let t12385 = t980 * t1258;
    let t12395 = 0.23707617058567841754e2_f64 * t3036 * t452 * t12235;
    let t12401 = t2937 * t929;
    let t12410 = t1160 * t3065 * t930;
    (t12360, t12385, t12395, t12401, t12410)
}
