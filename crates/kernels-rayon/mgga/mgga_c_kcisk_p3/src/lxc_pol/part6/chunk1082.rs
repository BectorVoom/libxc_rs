//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1082/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1082(t31776: f64, t31795: f64, t15094: f64, t1611: f64, t21345: f64, t2347: f64, t28036: f64, t31197: f64, t31199: f64, t31201: f64, t31203: f64, t31206: f64, t31209: f64, t31406: f64, t31744: f64, t31752: f64, t31755: f64, t4535: f64, t555: f64, t6604: f64, t8436: f64, t8455: f64) -> f64 {
    let t31796 = t31776 + t31795;
    let t31798 = -6.0_f64 * t15094 * t31752 - t1611 * t31796 + 6.0_f64 * t21345 * t8436 - 3.0_f64 * t2347 * t28036 + t31744 * t555 + 6.0_f64 * t31755 * t4535 - 3.0_f64 * t6604 * t8455 - t31197 + t31199 - t31201 + t31203 + t31206 - t31209 + t31406;
    t31798
}
