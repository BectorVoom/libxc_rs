//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1070/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1070(t120: f64, t6517: f64, t2225: f64, t10734: f64, t254: f64, t255: f64, t6314: f64, t6321: f64, t1415: f64, t2116: f64, t5: f64, t511: f64, t57: f64, t7: f64) -> (f64, f64, f64) {
    let t37816 = t120 * t6517;
    let t37817 = t37816 * t2225;
    let t37822 = t254 * t10734 * t6314 * t255 * t6321;
    let t37823 = 0.71120679974571020322e0_f64 * t37822;
    let t37833 = t5 * t7 * t1415 * t511 * t57 * t2116;
    (t37817, t37823, t37833)
}
