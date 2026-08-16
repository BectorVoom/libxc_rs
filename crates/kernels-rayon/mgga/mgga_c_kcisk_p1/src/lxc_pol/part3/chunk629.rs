//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 629/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk629(t2029: f64, t5520: f64, t1994: f64, t5075: f64, t5078: f64, t5080: f64, t5178: f64, t5189: f64, t5197: f64, t5201: f64, t5206: f64, t5432: f64, t5440: f64, t5445: f64, t795: f64) -> (f64, f64) {
    let t5521 = t5520 * t2029;
    let t5524 = 0.15476481481481481481e-2_f64 * t5075 + 0.23214722222222222222e-2_f64 * t5078 + 0.23214722222222222222e-2_f64 * t5080 + 0.17411041666666666666e-2_f64 * t5178 + t5432 * t795 - 0.23214722222222222222e-2_f64 * t5189 + 0.15476481481481481481e-2_f64 * t5197 - 0.23214722222222222222e-2_f64 * t5201 + 0.193e0_f64 * t1994 * t5440 + 0.74498e-1_f64 * t5445 * t5440 - 0.38691203703703703703e-3_f64 * t5206 - 0.193e0_f64 * t1994 * t5521;
    (t5521, t5524)
}
