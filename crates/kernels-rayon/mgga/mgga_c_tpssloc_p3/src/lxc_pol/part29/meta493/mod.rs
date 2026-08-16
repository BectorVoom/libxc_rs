//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta493 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1844;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1845;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta493(t3252: f64, t7286: f64, t7285: f64, t3248: f64, t24574: f64, t7288: f64, t225: f64, t7306: f64, t2154: f64, t3599: f64, t11606: f64, t11925: f64, t11928: f64, t1238: f64, t1252: f64, t2155: f64, t24630: f64, t24634: f64, t24639: f64, t24646: f64, t24758: f64, t24868: f64, t24871: f64, t24873: f64, t24877: f64, t24880: f64, t3593: f64, t3631: f64, t498: f64, t7283: f64, t7351: f64, t7392: f64, t265: f64, t504: f64, t24629: f64, t3640: f64, t7394: f64, t11947: f64, t2157: f64, t1254: f64, t1256: f64, t193: f64, t23772: f64, t336: f64, t3633: f64, t3637: f64, t4700: f64, t7398: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24883, t24884, t24887, t24888, t24891, t24893, t24897, t24900) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1844(t3252, t7286, t7285, t3248, t24574, t7288, t225, t7306, t2154, t3599, t11606, t11925, t11928, t1238, t1252, t2155, t24630, t24634, t24639, t24646, t24758, t24868, t24871, t24873, t24877, t24880, t3593, t3631, t498, t7283, t7351, t7392);
        let (t24901, t24905, t24909, t24916) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1845(t265, t504, t24629, t24900, t3640, t7394, t11947, t2157, t1254, t1256, t193, t23772, t336, t3633, t3637, t4700, t7398);
    (t24883, t24884, t24887, t24888, t24891, t24893, t24897, t24901, t24905, t24909, t24916)
}
