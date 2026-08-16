//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 838/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk838(t5202: f64, t5205: f64, t5209: f64, t5212: f64, t5213: f64, t5218: f64, t5220: f64, t5225: f64, t5230: f64, t5233: f64, t5237: f64, t3128: f64, t60: f64) -> (f64, f64) {
    let t8884 = -t5202 - t5205 - t5209 + t5212 + 0.26680286006577777776e-2_f64 * t5213 - t5218 - 0.33872559466666666666e-2_f64 * t5220 - t5225 + t5230 - t5233 - 0.10843581300301739842e-1_f64 * t5237;
    let t8892 = t60 * t3128;
    (t8884, t8892)
}
