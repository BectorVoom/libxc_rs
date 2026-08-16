//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1301/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1301(t24585: f64, t7999: f64, t11947: f64, t8090: f64, t27331: f64, t9239: f64, t45844: f64, t7245: f64, t1419: f64, t2274: f64, t111: f64, t8110: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95824 = t7999 * t24585;
    let t95925 = t8090 * t11947;
    let t96045 = t9239 * t27331;
    let t96120 = t45844 * t7245;
    let t96157 = t1419 * t2274;
    let t96334 = t8110 * t111;
    (t95824, t95925, t96045, t96120, t96157, t96334)
}
