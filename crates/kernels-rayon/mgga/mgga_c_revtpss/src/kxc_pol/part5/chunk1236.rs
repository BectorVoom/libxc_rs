//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1236/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1236(t19680: f64, t4806: f64, t1042: f64, t5819: f64, t999: f64, t1032: f64, t6235: f64, t1040: f64, t5825: f64, t4872: f64, t1651: f64, t905: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19687 = t4806 * t19680;
    let t19688 = t1042 * t19687;
    let t19691 = t5819 * t999;
    let t19692 = t4806 * t19691;
    let t19693 = t1042 * t19692;
    let t19696 = t6235 * t1032;
    let t19697 = t19696 * t1040;
    let t19700 = t5825 * t999;
    let t19701 = t4872 * t19700;
    let t19702 = t1042 * t19701;
    let t19705 = t1651 * t905;
    (t19688, t19691, t19693, t19697, t19702, t19705)
}
