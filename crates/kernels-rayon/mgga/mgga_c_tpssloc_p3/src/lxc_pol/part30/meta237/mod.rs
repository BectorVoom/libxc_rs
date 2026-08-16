//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta237 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1067;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1068;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1069;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta237(t475: f64, t6218: f64, t1214: f64, t248: f64, t1734: f64, t3508: f64, t1213: f64, t1227: f64, t1737: f64, t1748: f64, t3506: f64, t3515: f64, t3542: f64, t3547: f64, t467: f64, t5005: f64, t5019: f64, t5024: f64, t5036: f64, t5041: f64, t6109: f64, t6203: f64, t6207: f64, t6211: f64, t6197: f64, t466: f64, t1760: f64, t3598: f64, t491: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6219, t6221, t6224) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1067(t475, t6218, t1214, t248, t1734);
        let (t6225, t6227, t6230, t6232, t6237) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1068(t3508, t6224, t1214, t248, t475, t1213, t1227, t1737, t1748, t3506, t3515, t3542, t3547, t467, t5005, t5019, t5024, t5036, t5041, t6109, t6203, t6207, t6211, t6221);
        let (t6238, t6239, t6243, t6244, t6252) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1069(t6197, t6237, t466, t1760, t3598, t491, t6224);
    (t6219, t6221, t6224, t6225, t6227, t6230, t6232, t6238, t6239, t6243, t6244, t6252)
}
