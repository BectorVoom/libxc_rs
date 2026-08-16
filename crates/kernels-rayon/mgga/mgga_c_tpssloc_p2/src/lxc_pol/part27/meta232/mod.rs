//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta232 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1105;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1106;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1107;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta232(t3815: f64, t1788: f64, t588: f64, t592: f64, t3829: f64, t3833: f64, t2426: f64, t2486: f64, t3819: f64, t3821: f64, t3825: f64, t3827: f64, t3832: f64, t5169: f64, t225: f64, t5262: f64, t546: f64, t68: f64, t1365: f64, t1799: f64, t1307: f64, t1347: f64, t5187: f64, t1345: f64, t1348: f64, t1819: f64, t1821: f64, t548: f64, t550: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5263, t5265, t5267, t5268, t5269, t5270) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1105(t3815, t1788, t588, t592, t3829, t3833, t2426, t2486, t3819, t3821, t3825, t3827, t3832, t5169);
        let (t5272, t5278, t5279, t5280, t5283, t5286) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1106(t225, t5262, t5270, t546, t68, t1365, t1799, t1307, t1347, t5187, t1345, t1348, t1819, t1821, t548);
        let t5287 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1107(t5286, t550);
    (t5263, t5265, t5267, t5268, t5269, t5272, t5278, t5279, t5280, t5283, t5286, t5287)
}
