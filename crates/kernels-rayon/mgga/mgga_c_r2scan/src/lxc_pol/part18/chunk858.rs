//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 858/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk858(t4827: f64, t4839: f64, t4842: f64, t5000: f64, t5004: f64, t5008: f64, t5020: f64, t6010: f64, t6012: f64, t7025: f64, t8641: f64, t3128: f64, t424: f64) -> (f64, f64) {
    let t9055 = t5000 + t5004 + t5008 + t4827 - t4839 - t8641 + t5020 + t6010 - 0.571528e-1_f64 * t6012 - t4842 + t7025;
    let t9056 = t424 * t3128;
    (t9055, t9056)
}
