//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 686/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk686(t262: f64, t9888: f64, t7641: f64, t7648: f64, t9885: f64, t7653: f64, t3826: f64, t9708: f64, t3851: f64, t1707: f64, t649: f64, t7599: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9889 = t262 * t9888;
    let t9890 = t7641 * t9889;
    let t9892 = t7648 * t9885;
    let t9894 = t7653 * t9889;
    let t9897 = t3826 * t9708;
    let t9899 = t3851 * t9708;
    let t9903 = t649 * t1707;
    let t9904 = t7599 * t9903;
    (t9889, t9890, t9892, t9894, t9897, t9899, t9903, t9904)
}
