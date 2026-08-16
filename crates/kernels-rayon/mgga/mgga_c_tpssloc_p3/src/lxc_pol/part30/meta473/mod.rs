//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta473 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1767;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1768;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta473(t1049: f64, t362: f64, t225: f64, t23592: f64, t23384: f64, t6787: f64, t3216: f64, t6818: f64, t11094: f64, t1958: f64, t2752: f64, t28: f64, t112: f64, t7002: f64, t111: f64, t2022: f64, t1976: f64, t4072: f64, t671: f64, t7670: f64, t191: f64, t192: f64, t5118: f64, t2020: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23685, t23696, t23712, t23738, t23742, t23788) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1767(t1049, t362, t225, t23592, t23384, t6787, t3216, t6818, t11094, t1958, t2752, t28);
        let (t23877, t23880, t24980, t24983, t24987, t24988) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1768(t112, t7002, t111, t2022, t1976, t4072, t671, t7670, t191, t192, t5118, t2020);
    (t23685, t23696, t23712, t23738, t23742, t23788, t23877, t23880, t24980, t24983, t24987, t24988)
}
