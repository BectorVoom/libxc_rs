//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 656/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk656(t7583: f64, t8702: f64, t8706: f64, t8710: f64, t8714: f64, t8716: f64, t8718: f64, t8720: f64, t8722: f64, t8724: f64, t8726: f64, t797: f64, t8700: f64) -> (f64, f64) {
    let t8728 = -0.45457769423742236216e-2_f64 * t8702 + 0.9072038638458063915e-4_f64 * t8706 - 0.2419210303588817044e-3_f64 * t8710 + 0.28224120208536198848e-3_f64 * t8714 - 0.90915538847484472432e-2_f64 * t8716 + 0.12122071846331262991e-1_f64 * t8718 - 0.10584045078201074568e-3_f64 * t8720 + 0.34093327067806677162e-2_f64 * t8722 + 0.19914231157590872008e-2_f64 * t8724 - 0.27879923620627220811e-2_f64 * t8726 + t7583;
    let t8729 = t797 * t8700;
    (t8728, t8729)
}
