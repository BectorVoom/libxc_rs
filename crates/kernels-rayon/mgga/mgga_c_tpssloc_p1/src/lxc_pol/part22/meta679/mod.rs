//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta679 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2241;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2242;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta679(t1041: f64, t10868: f64, t248: f64, t5681: f64, t13965: f64, t4641: f64, t17659: f64, t3048: f64, t14207: f64, t4630: f64, t13969: f64, t17717: f64, t3039: f64, t1020: f64, t10508: f64, t5867: f64, t5878: f64, t17696: f64, t10422: f64, t17648: f64, t3070: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62137, t62148, t62150, t62152, t62164) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2241(t1041, t10868, t248, t5681, t13965, t4641, t17659, t3048, t14207, t4630, t13969, t17717, t3039);
        let (t62177, t62183, t62210, t62234) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2242(t1020, t10508, t248, t5867, t3039, t5878, t1041, t13969, t17696, t10422, t17648, t3070);
    (t62137, t62148, t62150, t62152, t62164, t62177, t62183, t62210, t62234)
}
