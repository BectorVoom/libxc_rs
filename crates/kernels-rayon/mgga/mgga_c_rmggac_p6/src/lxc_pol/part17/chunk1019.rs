//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1019/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1019(t46525: f64, t4669: f64, t1652: f64, t40940: f64, t41532: f64, t41535: f64, t41537: f64, t41550: f64, t46765: f64, t46770: f64, t46772: f64, t46774: f64, t46779: f64, t46782: f64, t5266: f64, t570: f64, t793: f64, t8946: f64) -> f64 {
    let t46784 = t4669 * t46525;
    let t46786 = t41532 - t41535 - t41537 + 0.39914139006212695213e-1_f64 * t46765 + 0.23948483403727617128e0_f64 * t5266 * t40940 * t570 - 0.8980681276397856423e-1_f64 * t46770 + 0.17961362552795712846e0_f64 * t46772 + 0.44903406381989282115e-1_f64 * t46774 + 0.23948483403727617128e0_f64 * t5266 * t8946 * t1652 + 0.11974241701863808564e0_f64 * t793 * t46779 - t41550 - 0.8980681276397856423e-1_f64 * t46782 + 0.8980681276397856423e-1_f64 * t46784;
    t46786
}
