//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 809/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk809(t5542: f64, t8601: f64, t674: f64, t2004: f64, t8607: f64, t7677: f64, t8571: f64, t2007: f64, t2310: f64, t36542: f64, t7720: f64, t8597: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t38350 = t8601 * t5542;
    let t38351 = t38350 * t674;
    let t38352 = t38351 * t2004;
    let t38354 = t8607 * t5542;
    let t38355 = t38354 * t674;
    let t38356 = t38355 * t2004;
    let t38358 = t8571 * t7677;
    let t38361 = t38351 * t2007;
    let t38363 = t38355 * t2007;
    let t38365 = t36542 * t2310;
    let t38367 = t7720 * t8597;
    (t38350, t38351, t38352, t38354, t38355, t38356, t38358, t38361, t38363, t38365, t38367)
}
