//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1265/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1265(t18210: f64, t28834: f64, t7978: f64, t1598: f64, t251: f64, t54605: f64, t98624: f64, t27601: f64, t28727: f64, t98637: f64, t27651: f64, t8209: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99494 = 0.23168402777777777778e-3_f64 * t7978 * t18210 * t28834;
    let t99497 = t54605 * t251 * t1598;
    let t99504 = 0.15476481481481481481e-2_f64 * t98624;
    let t99506 = 0.61782407407407407408e-3_f64 * t28727 * t27601;
    let t99512 = 0.15476481481481481481e-2_f64 * t98637;
    let t99524 = t8209 * t27651;
    (t99494, t99497, t99504, t99506, t99512, t99524)
}
