//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 937/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk937(t1516: f64, t30720: f64, t112802: f64, t32844: f64, t1499: f64, t30719: f64, t8344: f64, t32814: f64, t81651: f64, t82074: f64, t23168: f64, t32789: f64) -> (f64, f64, f64, f64, f64) {
    let t118588 = t30720 * t1516;
    let t118596 = t112802 * t32844;
    let t118602 = t1499 * t30719 * t8344;
    let t118632 = t81651 * t82074 * t32814;
    let t118649 = t23168 * t32789;
    (t118588, t118596, t118602, t118632, t118649)
}
