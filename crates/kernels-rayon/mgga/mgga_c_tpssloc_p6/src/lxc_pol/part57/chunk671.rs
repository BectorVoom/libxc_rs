//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 671/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk671(t3701: f64, t6463: f64, t562: f64, t6414: f64, t1824: f64, t1834: f64, t6387: f64, t120: f64, t225: f64, t6364: f64, t6435: f64, t6362: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19596 = t6463 * t3701;
    let t19660 = t562 * t6414;
    let t19739 = t1834 * t1824;
    let t19743 = t562 * t6387;
    let t19871 = t120 * t6387;
    let t19956 = t120 * t6414;
    let t20029 = t6364 * t225;
    let t20044 = t6435 * t225;
    let t20060 = t6362 * t225;
    (t19596, t19660, t19739, t19743, t19871, t19956, t20029, t20044, t20060)
}
