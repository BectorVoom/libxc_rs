//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1308/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1308(t4261: f64, t8343: f64, t32840: f64, t849: f64, t112802: f64, t32844: f64, t1499: f64, t30719: f64, t8344: f64, t1894: f64, t4119: f64, t59: f64, t6591: f64) -> (f64, f64, f64, f64, f64) {
    let t118592 = t8343 * t4261;
    let t118594 = t32840 * t849;
    let t118596 = t112802 * t32844;
    let t118602 = t1499 * t30719 * t8344;
    let t118606 = t6591 * t1894 * t59 * t4119;
    (t118592, t118594, t118596, t118602, t118606)
}
