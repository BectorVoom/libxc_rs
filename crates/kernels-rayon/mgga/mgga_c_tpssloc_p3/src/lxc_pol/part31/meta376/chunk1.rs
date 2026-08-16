//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1327/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1327(t1527: f64, t4300: f64, t2718: f64, t17050: f64, t17052: f64, t17057: f64, t17060: f64, t17064: f64, t259: f64, t2597: f64, t2713: f64, t4147: f64, t4268: f64, t4273: f64, t4301: f64, t5637: f64, t5658: f64, t855: f64, t866: f64) -> (f64, f64, f64) {
    let t17069 = t1527 * t4300;
    let t17070 = t2718 * t17069;
    let t17079 = -t17050 * t855 - t17052 * t866 + 2.0_f64 * t17057 * t855 + t17060 * t259 - 6.0_f64 * t17064 * t855 + 4.0_f64 * t17070 * t855 + 2.0_f64 * t2597 * t5637 + 2.0_f64 * t2713 * t5637 - t2713 * t5658 + 4.0_f64 * t4147 * t4273 - 2.0_f64 * t4147 * t4301 - 2.0_f64 * t4268 * t4301;
    (t17069, t17070, t17079)
}
