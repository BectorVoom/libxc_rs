//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1326/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1326(t1014: f64, t29383: f64, t102328: f64, t102698: f64, t102701: f64, t102706: f64, t102709: f64, t102712: f64, t102715: f64, t102723: f64, t18128: f64, t27583: f64, t27584: f64, t28701: f64, t77753: f64, t7978: f64, t99248: f64, t99646: f64) -> (f64, f64) {
    let t102725 = t1014 * t29383;
    let t102727 = 0.15476481481481481481e-2_f64 * t102698 - 0.30952962962962962962e-2_f64 * t102701 + 0.69505208333333333334e-3_f64 * t7978 * t102328 + 0.23214722222222222222e-2_f64 * t102706 - 0.15476481481481481481e-2_f64 * t102709 + 0.69644166666666666666e-2_f64 * t102712 + 0.92858888888888888888e-2_f64 * t102715 - 0.46336805555555555556e-3_f64 * t27583 * t18128 * t27584 * t77753 - 0.82448622685185185187e-4_f64 * t99248 * t28701 + 0.15476481481481481481e-2_f64 * t102723 + t99646 + 0.23214722222222222221e-2_f64 * t102725;
    (t102725, t102727)
}
