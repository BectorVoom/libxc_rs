//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta661 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2476;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2477;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2478;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2479;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2480;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2481;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta661<F: Float>(t265: F, t394: F, t49244: F, t49256: F, t49259: F, t49262: F, t49268: F, t49271: F, t49273: F, t49276: F, t49567: F, t49572: F, t49575: F, t47655: F, t49585: F, t50750: F, t50755: F, t50757: F, t50764: F, t50771: F, t50779: F, t25: F, t10150: F, t1074: F, t11105: F, t12606: F, t13493: F, t1408: F, t1409: F, t14675: F, t1534: F, t1642: F, t2249: F, t2250: F, t3220: F, t396: F, t3966: F, t40: F, t4324: F, t45872: F, t4705: F, t47668: F, t47670: F, t47672: F, t47674: F, t47676: F, t606: F, t607: F, t9257: F, t9258: F, dens_threshold: F, rho0: F, zeta_threshold: F, t11286: F, t4869: F, t1703: F, t43700: F, t11190: F, t1670: F, t11407: F, t3242: F, t457: F, t45971: F, t48140: F, t2394: F, t4734: F, t14707: F, t690: F, t1089: F, t1088: F, t123: F, t1654: F, t9698: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t50785 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2476::<F>(t265, t394, t49244, t49256, t49259, t49262, t49268, t49271, t49273, t49276, t49567, t49572, t49575, t47655, t49585, t50750, t50755, t50757, t50764, t50771, t50779);
        let t50803 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2477::<F>(t25, t10150, t1074, t11105, t12606, t13493, t1408, t1409, t14675, t1534, t1642, t2249, t2250, t3220, t396, t3966, t40, t4324, t45872, t4705, t47655, t47668, t47670, t47672, t47674, t47676, t50785, t606, t607, t9257, t9258, dens_threshold, rho0, zeta_threshold);
        let (t50816, t50818, t50821, t50824, t50826) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2478::<F>(t11286, t4869, t1703, t43700, t11190, t1670, t11407, t3242, t457, t45971, t48140, t2394, t4734);
        let (t50827, t50828) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2479::<F>(t50826, t14707, t690);
        let (t50830, t50832) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2480::<F>(t1089, t45872, t1088, t123);
        let t50834 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2481::<F>(t1654, t9698);
    (t50803, t50816, t50818, t50821, t50824, t50826, t50827, t50828, t50830, t50832, t50834)
}
