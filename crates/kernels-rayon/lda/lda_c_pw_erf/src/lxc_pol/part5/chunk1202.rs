//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1202/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1202(t17557: f64, t813: f64, t17785: f64, t21738: f64, t21739: f64, t21740: f64, t21741: f64, t21743: f64, t21746: f64, t21750: f64, t21752: f64, t21754: f64, t21757: f64, t21761: f64, t21764: f64) -> (f64, f64, f64) {
    let t21766 = 4.0_f64 / 5.0_f64 * t17557 * t813;
    let t21767 = 16.0_f64 / 45.0_f64 * t17785;
    let t21768 = t21738 - t21739 + t21740 + t21741 + t21743 + t21746 - t21750 - t21752 - t21754 - t21757 + t21761 + t21764 + t21766 + t21767;
    (t21766, t21767, t21768)
}
