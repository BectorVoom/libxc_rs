//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1433/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1433(t22237: f64, t4869: f64, t78242: f64, t78247: f64, t78250: f64, t78254: f64, t78281: f64, t78283: f64, t78286: f64, t78291: f64, t78294: f64, t78296: f64, t78298: f64, t78302: f64) -> (f64, f64) {
    let t78304 = 0.4101607543286562663e4_f64 * t4869 * t22237;
    let t78305 = t78242 - t78247 + t78250 + t78254 - t78281 - t78283 + t78286 - t78291 - t78294 + t78296 - t78298 + t78302 - t78304;
    (t78304, t78305)
}
