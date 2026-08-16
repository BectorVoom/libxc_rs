//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 522/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk522(t2600: f64, t541: f64, t1329: f64, t3726: f64, t1332: f64, t68: f64) -> (f64, f64, f64) {
    let t3762 = 35.0_f64 / 432.0_f64 * t2600 * t541;
    let t3763 = t3726 * t1329;
    let t3777 = t1332 * t68;
    (t3762, t3763, t3777)
}
