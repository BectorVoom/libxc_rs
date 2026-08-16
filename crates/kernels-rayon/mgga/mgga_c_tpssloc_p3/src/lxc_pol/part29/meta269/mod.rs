//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta269 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1261;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1262;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1263;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta269(t218: f64, t7510: f64, t1527: f64, t1911: f64, t2718: f64, t1484: f64, t6638: f64, t6637: f64, t6552: f64, t232: f64, t4282: f64, t6646: f64, t1888: f64, t1519: f64, t1894: f64, t214: f64, t1880: f64, t1510: f64, t6657: f64, t235: f64, t1499: f64, t1909: f64, t226: f64, t6636: f64, t6645: f64, t812: f64, t858: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7511, t7517, t7520, t7521, t7522, t7524, t7525) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1261(t218, t7510, t1527, t1911, t2718, t1484, t6638, t6637, t6552, t232, t4282, t6646);
        let (t7528, t7529, t7533, t7535, t7537) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1262(t1888, t7525, t1519, t1894, t214, t1880, t1510, t6657, t235, t7510, t1499, t1909, t226, t6636, t6645, t7522, t812);
        let t7538 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1263(t7537, t858);
    (t7511, t7517, t7520, t7521, t7524, t7525, t7528, t7529, t7533, t7535, t7537, t7538)
}
