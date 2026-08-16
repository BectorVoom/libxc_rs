//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 964/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk964(t1392: f64, t1979: f64, t1982: f64, t201: f64, t457: f64, t7428: f64, t8688: f64, t2004: f64, t9087: f64, t2412: f64, t7677: f64, t2007: f64) -> (f64, f64, f64, f64, f64) {
    let t40502 = t1392 * t457 * t201 * t1979 * t1982;
    let t40505 = t8688 * t7428 * t1982;
    let t40506 = 0.19863479950205658386e-4_f64 * t40505;
    let t40507 = t9087 * t2004;
    let t40509 = t2412 * t7677;
    let t40511 = t9087 * t2007;
    (t40502, t40506, t40507, t40509, t40511)
}
