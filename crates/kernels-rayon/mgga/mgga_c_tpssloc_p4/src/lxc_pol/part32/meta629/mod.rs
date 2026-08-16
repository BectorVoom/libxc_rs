//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta629 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2040;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2041;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta629(t87197: f64, t23145: f64, t4166: f64, t22690: f64, t234: f64, t7496: f64, t776: f64, t81792: f64, t23109: f64, t23110: f64, t232: f64, t236: f64, t4233: f64, t25132: f64, t81876: f64, t131: f64, t6598: f64, t9537: f64, t225: f64, t2627: f64, t25093: f64, t1512: f64, t81807: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87198, t87199, t87202, t87206, t87211) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2040(t87197, t23145, t4166, t22690, t234, t7496, t776, t81792, t23109, t23110, t232, t236, t4233);
        let (t87212, t87213, t87229, t87230, t87234, t87243) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2041(t87211, t25132, t81876, t131, t6598, t9537, t225, t2627, t236, t25093, t1512, t81807);
    (t87198, t87199, t87202, t87206, t87212, t87213, t87229, t87230, t87234, t87243)
}
