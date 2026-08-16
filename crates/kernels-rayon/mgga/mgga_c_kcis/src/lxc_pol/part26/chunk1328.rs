//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1328/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1328(t101950: f64, t102729: f64, t102731: f64, t102733: f64, t102735: f64, t102740: f64, t102743: f64, t102746: f64, t2256: f64, t2260: f64, t62923: f64, t7986: f64, t99667: f64, t99671: f64, t99676: f64) -> f64 {
    let t102751 = -0.23214722222222222221e-2_f64 * t102729 - 0.25794135802469135802e-3_f64 * t102731 - t99667 + 0.23168402777777777778e-3_f64 * t102733 + t99671 + 0.23168402777777777778e-3_f64 * t102735 + 0.33980324074074074074e-2_f64 * t101950 * t7986 + 0.92858888888888888886e-2_f64 * t102740 - 0.92858888888888888886e-2_f64 * t102743 + 0.17024129629629629629e-1_f64 * t102746 - 0.34752604166666666667e-3_f64 * t62923 * t2256 * t2260 + t99676;
    t102751
}
