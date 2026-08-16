//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 209/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk209(t224: f64, t898: f64, t806: f64, t810: f64, t813: f64, t816: f64, t819: f64, t824: f64) -> (f64, f64) {
    let t899 = t224 * t898;
    let t906 = 0.1875e0_f64 * t806 - 0.1875e0_f64 * t810 - 0.375e0_f64 * t813 - 0.4046875e-1_f64 * t816 + 0.4046875e-1_f64 * t819 + 0.161875e0_f64 * t824;
    (t899, t906)
}
