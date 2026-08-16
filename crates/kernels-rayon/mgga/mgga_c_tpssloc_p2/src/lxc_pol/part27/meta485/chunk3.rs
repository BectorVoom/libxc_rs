//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1865/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1865(t265: f64, t394: f64, t1068: f64, t1070: f64, t193: f64, t23734: f64, t23738: f64, t23742: f64, t23772: f64, t3209: f64, t3213: f64, t336: f64, t4700: f64, t6822: f64) -> f64 {
    let t395 = t265 < t394;
    let t23773 = piecewise3(t395, t1070 * t193 * t23734 * t336 - 2.0_f64 * t1068 * t23738 * t4700 + 2.0_f64 * t23742 * t3213 * t4700 - t3209 * t4700 * t6822, t23772);
    t23773
}
