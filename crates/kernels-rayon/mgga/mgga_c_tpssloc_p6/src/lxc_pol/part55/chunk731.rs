//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 731/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk731(t478: f64, t7327: f64, t1215: f64, t68: f64, t475: f64, t1202: f64, t2140: f64, t1209: f64, sigma2: f64) -> (f64, f64, f64, f64, f64) {
    let t7328 = t7327 * t478;
    let t7329 = t1215 * t68;
    let t7330 = t7329 * t475;
    let t7331 = t7328 * t7330;
    let t7334 = t1202 * t2140;
    let t7337 = t1209 * sigma2;
    (t7328, t7330, t7331, t7334, t7337)
}
