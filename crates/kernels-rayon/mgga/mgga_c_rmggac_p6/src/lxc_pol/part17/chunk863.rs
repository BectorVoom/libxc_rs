//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 863/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk863(t10280: f64, t38234: f64, t38235: f64, t38236: f64, t38237: f64, t38238: f64, t38239: f64, t7384: f64, t9309: f64, t9764: f64, t9767: f64, t9335: f64, t9336: f64, t9785: f64, t9787: f64, t9792: f64, t9794: f64, t9797: f64, t9801: f64, t9805: f64, t9809: f64, t9811: f64) -> (f64, f64) {
    let t44533 = -t38234 - t38235 + t10280 + t38236 - t38237 + t9764 + t38238 + t9309 - t9767 + t7384 + t38239;
    let t44540 = -t9785 - t9787 - t9792 + t9794 + t9797 - t9801 - t9805 + t9809 - t9811 + t9335 + t9336;
    (t44533, t44540)
}
