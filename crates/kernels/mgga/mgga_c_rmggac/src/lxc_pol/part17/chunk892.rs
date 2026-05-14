//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 892/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk892<F: Float>(t1756: F, t7778: F, t305: F, t45418: F, t5271: F, t46258: F, t5162: F, t46415: F, t4669: F, t1704: F, t2124: F, t27048: F, t46541: F, t46525: F, t1652: F, t40940: F, t41532: F, t41535: F, t41537: F, t41550: F, t5266: F, t570: F, t793: F, t8946: F) -> (F, F, F) {
    let t46764 = t7778 * t1756;
    let t46765 = t305 * t46764;
    let t46770 = t5271 * t45418;
    let t46772 = t5162 * t46258;
    let t46774 = t4669 * t46415;
    let t46779 = t2124 * t1704;
    let t46782 = t27048 * t46541;
    let t46784 = t4669 * t46525;
    let t46786 = t41532 - t41535 - t41537 + 0.39914139006212695213e-1 * t46765 + 0.23948483403727617128e0 * t5266 * t40940 * t570 - 0.8980681276397856423e-1 * t46770 + 0.17961362552795712846e0 * t46772 + 0.44903406381989282115e-1 * t46774 + 0.23948483403727617128e0 * t5266 * t8946 * t1652 + 0.11974241701863808564e0 * t793 * t46779 - t41550 - 0.8980681276397856423e-1 * t46782 + 0.8980681276397856423e-1 * t46784;
    (t46764, t46779, t46786)
}
