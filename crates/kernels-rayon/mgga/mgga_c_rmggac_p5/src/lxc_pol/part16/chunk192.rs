//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 192/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk192(t655: f64, t656: f64, t344: f64, t36: f64, t22: f64, t349: f64, t194: f64, t202: f64) -> (f64, f64, f64, f64, f64) {
    let t657 = t655 * t656;
    let t659 = t344 * t36;
    let t661 = t349 * t22;
    let t662 = t661 * t656;
    let t671 = t194 * t202;
    (t657, t659, t661, t662, t671)
}
