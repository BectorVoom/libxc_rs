//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 933/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk933(t1982: f64, t7428: f64, t8688: f64, t2004: f64, t9087: f64, t2412: f64, t7677: f64, t2007: f64, t2286: f64, t7944: f64, t1627: f64, t2064: f64, t3928: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40505 = t8688 * t7428 * t1982;
    let t40507 = t9087 * t2004;
    let t40509 = t2412 * t7677;
    let t40511 = t9087 * t2007;
    let t40513 = t7944 * t2286;
    let t40516 = t3928 * t2064 * t1627;
    (t40505, t40507, t40509, t40511, t40513, t40516)
}
