//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 517/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk517(t4175: f64, t486: f64, t1297: f64, t391: f64, t494: f64, t79: f64, t499: f64, t493: f64, t1512: f64, t1516: f64, t3732: f64, t498: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4176 = t486 * t4175;
    let t4180 = 1.0_f64 / t391 / t494 / t1297;
    let t4181 = t4180 * t79;
    let t4182 = t4181 * t499;
    let t4183 = t493 * t4182;
    let t4185 = t1512 * t1516;
    let t4186 = t493 * t4185;
    let t4188 = t499 * t3732;
    let t4189 = t498 * t4188;
    (t4176, t4181, t4182, t4183, t4185, t4186, t4188, t4189)
}
