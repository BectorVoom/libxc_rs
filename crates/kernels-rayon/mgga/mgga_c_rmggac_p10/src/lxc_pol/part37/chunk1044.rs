//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1044/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1044(t1356: f64, t14980: f64, t5144: f64, t5267: f64, t5888: f64, t70797: f64, t73569: f64, t739: f64, t73936: f64, t76779: f64, t76780: f64, t76781: f64, t76787: f64, t76790: f64, t76792: f64, t76794: f64, t76796: f64, t76799: f64, t76800: f64, t76801: f64, t76802: f64, t884: f64) -> f64 {
    let t80014 = 0.87596530464506835932e-6_f64 * t73936 + t76779 - t76780 - t76781 + t76787 + t76790 - t76792 + t70797 + t76794 + t76796 + t76799 + 0.11974241701863808564e0_f64 * t739 * t14980 * t5144 - 0.11974241701863808564e0_f64 * t884 * t14980 * t5267 - 0.11974241701863808564e0_f64 * t1356 * t73569 * t5888 + t76800 + t76801 - t76802;
    t80014
}
