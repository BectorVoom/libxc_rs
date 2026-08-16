//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1031/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1031(t14607: f64, t10477: f64, t14619: f64, t14624: f64, t14631: f64, t14635: f64, t14638: f64, t14642: f64, t14644: f64, t14647: f64, t14652: f64, t15469: f64, t430: f64) -> (f64, f64) {
    let t15671 = 0.15476481481481481481e-2_f64 * t14607;
    let t15686 = -0.10446625e-1_f64 * t14619 - 0.18571777777777777777e-1_f64 * t14624 + t15469 * t430 + 0.46429444444444444443e-2_f64 * t14631 - 0.15476481481481481481e-2_f64 * t14635 - 0.15476481481481481481e-2_f64 * t10477 - 0.23214722222222222222e-2_f64 * t14638 + 0.69644166666666666666e-2_f64 * t14642 + 0.15476481481481481481e-2_f64 * t14644 - 0.41270617283950617284e-2_f64 * t14647 + 0.77382407407407407406e-3_f64 * t14652;
    (t15671, t15686)
}
