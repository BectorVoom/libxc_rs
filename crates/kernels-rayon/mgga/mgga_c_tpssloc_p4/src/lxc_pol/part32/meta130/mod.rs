//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta130 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk735;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk736;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk737;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta130(t3358: f64, t3236: f64, t1143: f64, t1147: f64, t1146: f64, t445: f64, t440: f64, t3293: f64, t448: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3359, t3363, t3371, t3375) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk735(t3358, t3236, t1143, t1147, t1146, t445);
        let (t3376, t3383, t3390, t3399, t3400) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk736(t3375, t440, t3236, t3293, t1146);
        let (t3401, t3402, t3403) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk737(t3400, t440, t448);
    (t3359, t3363, t3371, t3375, t3376, t3383, t3390, t3399, t3400, t3401, t3402, t3403)
}
