//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1042/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1042(t73857: f64, t73862: f64, t76713: f64, t76718: f64, t76723: f64, t76728: f64, t76733: f64, t76738: f64, t76743: f64, t76744: f64, t76745: f64, t76748: f64, t76749: f64, t76750: f64, t76751: f64, t76752: f64, t76753: f64) -> f64 {
    let t79993 = -t76713 + t76718 - t76723 + t76728 - t76733 - t76738 + t76743 + t76744 - t76745 - 0.87596530464506835932e-6_f64 * t73857 + 0.87596530464506835932e-6_f64 * t73862 - t76748 - t76749 + t76750 - t76751 + t76752 + t76753;
    t79993
}
