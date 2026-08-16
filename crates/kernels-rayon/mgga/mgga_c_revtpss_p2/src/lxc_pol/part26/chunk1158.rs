//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1158/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1158(t26205: f64, t6954: f64, t45958: f64, t7342: f64, t25110: f64, t26179: f64, t26169: f64, t6963: f64, t45963: f64, t2048: f64, t25102: f64, t25159: f64, t26172: f64, t26175: f64, t26187: f64, t6960: f64, t7343: f64, t7352: f64, t92639: f64, t92654: f64, t92696: f64, t92709: f64) -> f64 {
    let t95255 = t6954 * t26205;
    let t95259 = t45958 * t7342;
    let t95268 = t26179 * t25110;
    let t95270 = t6963 * t26169;
    let t95276 = t45963 * t7342;
    let t95281 = 88.0_f64 / 9.0_f64 * t95255 - 2.0_f64 * t92639 * t2048 - 5.0_f64 * t95259 * t6960 - 2.0_f64 * t92709 * t2048 - 10.0_f64 * t26187 * t25110 - 4.0_f64 * t25102 * t7352 + 80.0_f64 / 3.0_f64 * t95268 + 32.0_f64 / 3.0_f64 * t95270 - 5.0_f64 * t7343 * t92654 - 2.0_f64 * t6963 * t26172 + 30.0_f64 * t95276 * t25159 + 30.0_f64 * t26175 * t92696;
    t95281
}
