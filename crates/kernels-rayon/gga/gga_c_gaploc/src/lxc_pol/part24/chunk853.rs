//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 853/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk853(t2779: f64, t4673: f64, t188: f64, t7937: f64, t1457: f64, t7963: f64, t2788: f64, t4614: f64, t2846: f64, t1392: f64, t2778: f64, t1391: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8347 = t4673 * t2779;
    let t8352 = t188 * t7937;
    let t8355 = t1457 * t7963;
    let t8358 = t4614 * t2788;
    let t8361 = t4614 * t2846;
    let t8366 = t1392 * t2778;
    let t8367 = t1391 * t8366;
    (t8347, t8352, t8355, t8358, t8361, t8367)
}
