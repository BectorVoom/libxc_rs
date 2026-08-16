//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 502/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk502(t1763: f64, t4616: f64, t352: f64, t27: f64, t29: f64, t5840: f64, t3908: f64) -> (f64, f64) {
    let t6362 = t4616 * t1763;
    let t6363 = t6362 * t352;
    let t6374 = t5840 * t29 * t27;
    let t6376 = 5.0_f64 / 18.0_f64 * t6374 - t3908;
    (t6363, t6376)
}
