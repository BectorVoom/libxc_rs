//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1045/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1045(t14054: f64, t14117: f64, t14146: f64, t14430: f64, t1402: f64, t2: f64, t555: f64, t3765: f64, t3807: f64, t4844: f64, t8737: f64, t2476: f64, t4876: f64) -> (f64, f64, f64, f64, f64) {
    let t14432 = t14054 + t14117 + t14146 + t14430;
    let t14438 = t1402 * t2;
    let t14440 = 2.0_f64 * t14438 * t555;
    let t14447 = 2.0_f64 * t3765 * t3807;
    let t14449 = 2.0_f64 * t8737 * t4844;
    let t14451 = 1.0_f64 * t2476 * t4876;
    (t14432, t14440, t14447, t14449, t14451)
}
