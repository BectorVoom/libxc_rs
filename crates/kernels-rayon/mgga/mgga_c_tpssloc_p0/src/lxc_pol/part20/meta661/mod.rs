//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta661 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2476;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2477;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2478;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2479;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2480;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2481;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta661(t265: f64, t394: f64, t49244: f64, t49256: f64, t49259: f64, t49262: f64, t49268: f64, t49271: f64, t49273: f64, t49276: f64, t49567: f64, t49572: f64, t49575: f64, t47655: f64, t49585: f64, t50750: f64, t50755: f64, t50757: f64, t50764: f64, t50771: f64, t50779: f64, t25: f64, t10150: f64, t1074: f64, t11105: f64, t12606: f64, t13493: f64, t1408: f64, t1409: f64, t14675: f64, t1534: f64, t1642: f64, t2249: f64, t2250: f64, t3220: f64, t396: f64, t3966: f64, t40: f64, t4324: f64, t45872: f64, t4705: f64, t47668: f64, t47670: f64, t47672: f64, t47674: f64, t47676: f64, t606: f64, t607: f64, t9257: f64, t9258: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t11286: f64, t4869: f64, t1703: f64, t43700: f64, t11190: f64, t1670: f64, t11407: f64, t3242: f64, t457: f64, t45971: f64, t48140: f64, t2394: f64, t4734: f64, t14707: f64, t690: f64, t1089: f64, t1088: f64, t123: f64, t1654: f64, t9698: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t50785 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2476(t265, t394, t49244, t49256, t49259, t49262, t49268, t49271, t49273, t49276, t49567, t49572, t49575, t47655, t49585, t50750, t50755, t50757, t50764, t50771, t50779);
        let t50803 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2477(t25, t10150, t1074, t11105, t12606, t13493, t1408, t1409, t14675, t1534, t1642, t2249, t2250, t3220, t396, t3966, t40, t4324, t45872, t4705, t47655, t47668, t47670, t47672, t47674, t47676, t50785, t606, t607, t9257, t9258, dens_threshold, rho0, zeta_threshold);
        let (t50816, t50818, t50821, t50824, t50826) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2478(t11286, t4869, t1703, t43700, t11190, t1670, t11407, t3242, t457, t45971, t48140, t2394, t4734);
        let (t50827, t50828) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2479(t50826, t14707, t690);
        let (t50830, t50832) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2480(t1089, t45872, t1088, t123);
        let t50834 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2481(t1654, t9698);
    (t50803, t50816, t50818, t50821, t50824, t50826, t50827, t50828, t50830, t50832, t50834)
}
