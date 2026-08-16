//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1045/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1045(t68409: f64, t70799: f64, t73971: f64, t73974: f64, t73977: f64, t73981: f64, t73984: f64, t73994: f64, t74008: f64, t76803: f64, t76804: f64, t76805: f64, t76808: f64, t76814: f64, t76816: f64, t76817: f64, t76820: f64) -> f64 {
    let t80022 = -t76803 + t76804 + t76805 + t68409 + t70799 + t76808 + 0.52557918278704101558e-5_f64 * t73971 - 0.52557918278704101558e-5_f64 * t73974 - 0.17519306092901367186e-5_f64 * t73977 + 0.17519306092901367186e-5_f64 * t73981 - 0.17519306092901367186e-5_f64 * t73984 + t76814 - 0.17451485956252114153e-4_f64 * t73994 + t76816 - t76817 + 0.72714524817717142305e-5_f64 * t74008 + t76820;
    t80022
}
