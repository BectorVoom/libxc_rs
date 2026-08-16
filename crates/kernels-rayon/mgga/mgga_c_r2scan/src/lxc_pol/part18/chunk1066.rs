//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1066/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1066(t10734: f64, t254: f64, t255: f64, t6314: f64, t6321: f64, t1415: f64, t2116: f64, t5: f64, t511: f64, t57: f64, t7: f64, t2158: f64, t37699: f64) -> (f64, f64, f64) {
    let t37822 = t254 * t10734 * t6314 * t255 * t6321;
    let t37823 = 0.71120679974571020322e0_f64 * t37822;
    let t37833 = t5 * t7 * t1415 * t511 * t57 * t2116;
    let t37834 = 0.89443204944342177673e-3_f64 * t37833;
    let t37835 = t37699 * t2158;
    (t37823, t37834, t37835)
}
