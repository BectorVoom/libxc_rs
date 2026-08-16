//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta625 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2026;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2027;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta625(t252: f64, t4119: f64, t25160: f64, t814: f64, t22690: f64, t7520: f64, t81573: f64, t25324: f64, t6562: f64, t794: f64, t23030: f64, t25258: f64, t22893: f64, t23164: f64, t25306: f64, t7524: f64, t81612: f64, t81613: f64, t4250: f64, t81749: f64, t23145: f64, t4166: f64, t234: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87130, t87135, t87140, t87154, t87155) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2026(t252, t4119, t25160, t814, t22690, t7520, t81573, t25324, t6562, t794, t23030, t25258);
        let (t87166, t87177, t87198, t87199, t87202) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2027(t22893, t23164, t25306, t7524, t81612, t81613, t4250, t81749, t23145, t4166, t22690, t234);
    (t87130, t87135, t87140, t87154, t87155, t87166, t87177, t87198, t87199, t87202)
}
