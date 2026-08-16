//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 571/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk571(t3787: f64, t68: f64, t544: f64, t1824: f64, t562: f64, t1338: f64, t1834: f64, t112: f64, t1851: f64) -> (f64, f64, f64, f64, f64) {
    let t5333 = t68 * t3787;
    let t5334 = t544 * t5333;
    let t5335 = t562 * t1824;
    let t5343 = t68 * t1338;
    let t5344 = t544 * t5343;
    let t5348 = t1338 * t1834;
    let t5371 = t1851 * t112;
    (t5334, t5335, t5344, t5348, t5371)
}
