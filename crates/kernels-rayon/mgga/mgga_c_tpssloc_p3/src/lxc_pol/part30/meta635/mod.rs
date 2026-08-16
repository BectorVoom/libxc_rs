//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta635 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2044;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2045;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta635(t25192: f64, t81651: f64, t82074: f64, t225: f64, t25220: f64, t25054: f64, t23030: f64, t25205: f64, t23164: f64, t7479: f64, t82133: f64, t23204: f64, t25216: f64, t6562: f64, t1519: f64, t212: f64, t23171: f64, t6554: f64, t25040: f64, t87712: f64, t25193: f64, t81591: f64, t10143: f64, t7540: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87836, t87837, t87874, t87898, t87902, t87910) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2044(t25192, t81651, t82074, t225, t25220, t25054, t23030, t25205, t23164, t7479, t82133, t23204, t25216, t6562);
        let (t87911, t87915, t87927, t87932, t87975) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2045(t87910, t1519, t212, t23171, t6554, t25040, t82074, t87712, t25193, t81591, t10143, t7540);
    (t87836, t87837, t87874, t87898, t87902, t87911, t87915, t87927, t87932, t87975)
}
