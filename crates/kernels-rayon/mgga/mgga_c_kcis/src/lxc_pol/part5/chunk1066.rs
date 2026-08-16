//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1066/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1066(t1628: f64, t6220: f64, t15844: f64, t3947: f64, t5613: f64, t1911: f64, t3944: f64, t2072: f64, t4355: f64, t4330: f64, t16144: f64, t16048: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17710 = t6220 * t1628;
    let t17739 = 0.15476481481481481481e-2_f64 * t15844;
    let t17771 = t5613 * t3947;
    let t17784 = t1911 * t3944;
    let t17797 = t2072 * t4355;
    let t17834 = t2072 * t4330;
    let t17847 = 0.27785333333333333334e0_f64 * t16144;
    let t17856 = 0.22954444444444444444e0_f64 * t16048;
    (t17710, t17739, t17771, t17784, t17797, t17834, t17847, t17856)
}
