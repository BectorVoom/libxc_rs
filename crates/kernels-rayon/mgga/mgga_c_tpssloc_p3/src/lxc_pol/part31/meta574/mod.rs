//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta574 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1809;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1810;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta574(t25055: f64, t81591: f64, t25217: f64, t6547: f64, t25060: f64, t82209: f64, t82211: f64, t25192: f64, t81651: f64, t82074: f64, t82259: f64, t25054: f64, t23030: f64, t25205: f64, t23164: f64, t7479: f64, t82133: f64, t23204: f64, t25216: f64, t6562: f64, t1519: f64, t212: f64, t23171: f64, t6554: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87786, t87796, t87804, t87806, t87807, t87835, t87847, t87873) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1809(t25055, t81591, t25217, t6547, t25060, t82209, t82211, t25192, t81651, t82074, t82259, t25054);
        let (t87898, t87901, t87910, t87915) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1810(t23030, t25205, t23164, t7479, t82133, t23204, t25216, t6562, t1519, t212, t23171, t6554);
    (t87786, t87796, t87804, t87806, t87807, t87835, t87847, t87873, t87898, t87901, t87910, t87915)
}
