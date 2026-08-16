//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta677 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2237;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2238;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta677(t13961: f64, t4641: f64, t14137: f64, t4644: f64, t1041: f64, t13969: f64, t17971: f64, t17713: f64, t3130: f64, t17997: f64, t3070: f64, t42488: f64, t17975: f64, t17687: f64, t14085: f64, t4571: f64, t13765: f64, t13995: f64, t18086: f64, t3069: f64, t10952: f64, t17655: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61794, t61796, t61853, t61866, t61916) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2237(t13961, t4641, t14137, t4644, t1041, t13969, t17971, t17713, t3130, t17997, t3070, t42488);
        let (t61919, t61923, t61929, t61940, t61950, t61975) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2238(t1041, t13969, t17975, t17687, t14085, t4571, t13765, t13995, t18086, t3069, t10952, t17655);
    (t61794, t61796, t61853, t61866, t61916, t61919, t61923, t61929, t61940, t61950, t61975)
}
