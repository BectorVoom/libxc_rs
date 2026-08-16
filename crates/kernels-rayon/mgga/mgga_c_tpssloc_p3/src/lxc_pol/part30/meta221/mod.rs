//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta221 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1023;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1024;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1025;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1026;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1027;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1028;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta221(t5838: f64, t974: f64, t1597: f64, t340: f64, t343: f64, t2969: f64, t2986: f64, t4507: f64, t4529: f64, t5818: f64, t5821: f64, t5825: f64, t5829: f64, t973: f64, t381: f64, t1603: f64, t1625: f64, t1044: f64, t248: f64, t5685: f64, t3062: f64, t5677: f64, t5691: f64, t5693: f64, t5697: f64, t5729: f64, t5732: f64, t5798: f64, t5800: f64, t5802: f64, t5806: f64, t5810: f64, t5814: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5839, t5842) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1023(t5838, t974, t1597);
        let (t5843, t5844) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1024(t340, t5842, t343);
        let (t5845, t5848) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1025(t5844, t974, t2969, t2986, t4507, t4529, t5818, t5821, t5825, t5829, t5839, t973);
        let (t5849, t5851, t5857) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1026(t381, t5848, t1603, t1625, t1044, t248, t5685);
        let t5861 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1027(t248, t3062, t5677);
        let t5866 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1028(t5691, t5693, t5697, t5729, t5732, t5798, t5800, t5802, t5806, t5810, t5814);
    (t5839, t5842, t5843, t5844, t5845, t5848, t5849, t5851, t5857, t5861, t5866)
}
