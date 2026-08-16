//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 652/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk652(t10436: f64, t5203: f64, t1800: f64, t1869: f64, t1060: f64, t3290: f64) -> (f64, f64) {
    let t10437 = t5203 * t10436;
    let t10438 = t1800 * t10437;
    let t10439 = t1869 * t10438;
    let t10441 = t3290 * t1060;
    (t10439, t10441)
}
