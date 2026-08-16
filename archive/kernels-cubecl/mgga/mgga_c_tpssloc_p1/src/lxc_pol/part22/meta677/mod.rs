//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta677 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2237;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2238;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta677<F: Float>(t13961: F, t4641: F, t14137: F, t4644: F, t1041: F, t13969: F, t17971: F, t17713: F, t3130: F, t17997: F, t3070: F, t42488: F, t17975: F, t17687: F, t14085: F, t4571: F, t13765: F, t13995: F, t18086: F, t3069: F, t10952: F, t17655: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t61794, t61796, t61853, t61866, t61916) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2237::<F>(t13961, t4641, t14137, t4644, t1041, t13969, t17971, t17713, t3130, t17997, t3070, t42488);
        let (t61919, t61923, t61929, t61940, t61950, t61975) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2238::<F>(t1041, t13969, t17975, t17687, t14085, t4571, t13765, t13995, t18086, t3069, t10952, t17655);
    (t61794, t61796, t61853, t61866, t61916, t61919, t61923, t61929, t61940, t61950, t61975)
}
