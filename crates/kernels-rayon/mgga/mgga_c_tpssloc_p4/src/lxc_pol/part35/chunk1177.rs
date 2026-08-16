//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1177/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1177(t11605: f64, t2154: f64, t225: f64, t8055: f64, t460: f64, t491: f64, t7286: f64, t7280: f64, t7999: f64, t1170: f64, t8010: f64, t2121: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27785 = t11605 * t2154;
    let t27792 = t8055 * t225;
    let t27798 = t460 * t491;
    let t27799 = t27798 * t7286;
    let t27808 = t7999 * t7280;
    let t27817 = t1170 * t8010;
    let t27818 = t2121 * t27817;
    (t27785, t27792, t27799, t27808, t27817, t27818)
}
