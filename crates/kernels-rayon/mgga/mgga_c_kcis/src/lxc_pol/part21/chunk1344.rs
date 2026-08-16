//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1344/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1344(t26955: f64, t26960: f64, t26977: f64, t27020: f64, t28204: f64, t95569: f64, t95579: f64, t95581: f64, t95585: f64, t95626: f64, t96779: f64, t96781: f64, t96787: f64, t96790: f64, t96795: f64, t96799: f64) -> f64 {
    let t96802 = 0.69644166666666666666e-2_f64 * t95569 + t96779 + t96781 + 0.51588271604938271604e-3_f64 * t95579 - 0.41270617283950617284e-2_f64 * t95581 + 0.46377350260416666667e-4_f64 * t28204 * t27020 + 0.46429444444444444443e-2_f64 * t95585 - t96787 - 0.38691203703703703703e-3_f64 * t95626 - 0.92835860883789062501e-5_f64 * t96790 * t26977 + 0.41224311342592592592e-4_f64 * t26955 * t96795 - 0.23168402777777777778e-3_f64 * t26960 * t96799;
    t96802
}
