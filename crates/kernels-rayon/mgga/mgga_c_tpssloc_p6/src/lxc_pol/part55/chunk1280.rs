//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1280/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1280(t27650: f64, t8875: f64, t27680: f64, t1222: f64, t34272: f64, t34266: f64, t118017: f64, t118019: f64, t1201: f64, t24650: f64, t27684: f64, t27691: f64, t27711: f64, t32432: f64, t32433: f64, t34263: f64, t34271: f64, t488: f64, t4964: f64, t7326: f64, t8878: f64) -> f64 {
    let t125483 = t27650 * t8875;
    let t125485 = t27680 * t8875;
    let t125488 = t34272 * t1222;
    let t125492 = t34266 * t1222;
    let t125508 = -0.40372756094140390856e-3_f64 * t125483 - 0.32298204875312312685e-2_f64 * t125485 + 0.40372756094140390856e-3_f64 * t118017 - t125488 / 432.0_f64 - 0.40372756094140390856e-3_f64 * t27684 * t32433 + t125492 / 2304.0_f64 - t1201 * t34271 * t488 / 288.0_f64 - 0.32298204875312312685e-2_f64 * t27711 * t32433 + 0.40372756094140390856e-3_f64 * t7326 * t32432 * t27691 - 0.40372756094140390856e-3_f64 * t24650 * t34263 + t4964 * t8878 * t488 / 1536.0_f64 - 0.40372756094140390856e-3_f64 * t118019;
    t125508
}
