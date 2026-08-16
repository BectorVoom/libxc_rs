//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 388/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk388(t2521: f64, t706: f64, t1421: f64, t1875: f64, t2399: f64, t2514: f64, t2518: f64, t456: f64, t604: f64) -> (f64, f64) {
    let t2522 = t706 * t2521;
    let t2527 = t1875 + 0.65704296666666666667e-3_f64 * t1421 * t2514 + 0.1478346675e-2_f64 * t456 * t2518 - 0.98556445e-3_f64 * t456 * t2522 - 4.0_f64 * t604 * t2399;
    (t2522, t2527)
}
