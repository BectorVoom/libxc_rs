//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 824/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk824(t74829: f64, t14240: f64, t73692: f64, t11644: f64, t14236: f64, t2067: f64, t70397: f64, t11648: f64, t68626: f64, t68854: f64, t73793: f64, t68856: f64) -> (f64, f64, f64, f64, f64) {
    let t74830 = 0.23948483403727617128e0_f64 * t74829;
    let t74831 = t73692 * t14240;
    let t74835 = t14236 * t70397 * t2067 * t11644;
    let t74839 = t14236 * t68626 * t2067 * t11648;
    let t74841 = t73793 * t68854;
    let t74842 = t74841 * t68856;
    (t74830, t74831, t74835, t74839, t74842)
}
