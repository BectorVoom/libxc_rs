//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 820/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk820(t3131: f64, t5872: f64, t1021: f64, t248: f64, t360: f64, t3151: f64, t5392: f64, t974: f64, t5398: f64, t998: f64, t3146: f64, t1044: f64, t5681: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5873 = t5872 * t3131;
    let t5875 = t248 * t1021 * t5873;
    let t5878 = t5872 * t360;
    let t5880 = t248 * t1021 * t5878;
    let t5884 = t3151 * t5392;
    let t5885 = t974 * t5884;
    let t5889 = t998 * t5398;
    let t5890 = t974 * t5889;
    let t5893 = t3146 * t5392;
    let t5894 = t974 * t5893;
    let t5900 = t248 * t1044 * t5681;
    (t5873, t5875, t5878, t5880, t5884, t5885, t5889, t5890, t5893, t5894, t5900)
}
