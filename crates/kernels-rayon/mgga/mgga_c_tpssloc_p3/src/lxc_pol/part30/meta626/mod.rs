//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta626 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2028;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2029;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta626(t7496: f64, t776: f64, t81792: f64, t87202: f64, t23109: f64, t23110: f64, t232: f64, t236: f64, t4233: f64, t25132: f64, t81876: f64, t131: f64, t6598: f64, t9537: f64, t225: f64, t2627: f64, t25093: f64, t1512: f64, t81807: f64, t81824: f64, t23041: f64, t4236: f64, t23040: f64, t4166: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87206, t87212, t87213, t87229) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2028(t7496, t776, t81792, t87202, t23109, t23110, t232, t236, t4233, t25132, t81876, t131, t6598, t9537);
        let (t87230, t87234, t87243, t87248, t87256, t87261) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2029(t225, t2627, t236, t25093, t87229, t1512, t81807, t81824, t23041, t4236, t23040, t4166);
    (t87206, t87212, t87213, t87229, t87230, t87234, t87243, t87248, t87256, t87261)
}
