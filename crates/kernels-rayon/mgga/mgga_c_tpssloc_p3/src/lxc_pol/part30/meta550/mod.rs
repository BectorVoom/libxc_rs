//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta550 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1904;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1905;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta550(t3788: f64, t6388: f64, t6936: f64, t1339: f64, t6420: f64, t6417: f64, t6945: f64, t1827: f64, t26233: f64, t6415: f64, t22839: f64, t6371: f64, t1998: f64, t236: f64, t6330: f64, t22845: f64, t6347: f64, t6926: f64, t6375: f64, t6916: f64, t26246: f64, t26268: f64, t27012: f64, t27019: f64, t27022: f64, t27027: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t28057, t28058, t28060, t28061, t28063, t28065, t28067, t28068, t28070) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1904(t3788, t6388, t6936, t1339, t6420, t6417, t6945, t1827, t26233, t6415, t22839, t6371);
        let (t28073, t28077, t28083) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1905(t1998, t236, t6330, t22845, t6347, t6926, t6375, t6916, t26246, t26268, t27012, t27019, t27022, t27027, t28058, t28061, t28063, t28065, t28068, t28070);
    (t28057, t28060, t28067, t28073, t28077, t28083)
}
