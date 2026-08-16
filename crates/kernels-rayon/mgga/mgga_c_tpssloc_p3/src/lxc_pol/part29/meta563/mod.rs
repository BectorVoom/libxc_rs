//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1972;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1973;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta563(t460: f64, t491: f64, t7286: f64, t27453: f64, t27721: f64, t466: f64, t7280: f64, t7999: f64, t1186: f64, t8010: f64, t1170: f64, t2121: f64, t8034: f64, t7287: f64, t24567: f64, t8014: f64, t225: f64, t8018: f64, t1252: f64, t15797: f64, t2155: f64, t24589: f64, t24891: f64, t3487: f64, t4945: f64, t498: f64, t5055: f64, t5089: f64, t7283: f64, t7296: f64, t7351: f64, t7356: f64, t7392: f64, t8088: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27798, t27799, t27800, t27805, t27808, t27812, t27818) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1972(t460, t491, t7286, t27453, t27721, t466, t7280, t7999, t1186, t8010, t1170, t2121);
        let (t27820, t27821, t27826, t27830, t27832) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1973(t491, t8034, t7287, t24567, t8014, t225, t8018, t1252, t15797, t2155, t24589, t24891, t27800, t27805, t27808, t27812, t27818, t3487, t4945, t498, t5055, t5089, t7283, t7296, t7351, t7356, t7392, t7999, t8088);
    (t27798, t27799, t27800, t27805, t27812, t27820, t27821, t27826, t27830, t27832)
}
