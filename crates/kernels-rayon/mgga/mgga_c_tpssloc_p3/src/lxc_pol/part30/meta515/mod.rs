//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta515 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1843;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1844;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta515(t25374: f64, t25927: f64, t1081: f64, t1530: f64, t28: f64, t4303: f64, t1649: f64, t776: f64, t868: f64, t1877: f64, t1915: f64, t22959: f64, t23290: f64, t25013: f64, t2522: f64, t25354: f64, t25358: f64, t25372: f64, t25397: f64, t25892: f64, t25898: f64, t25901: f64, t25905: f64, t25921: f64, t6666: f64, t6670: f64, t6841: f64, t6848: f64, t7541: f64, t7649: f64, t7656: f64, t265: f64, t504: f64, t25882: f64, t1409: f64, t1972: f64, t3966: f64, t52: f64, t607: f64, t6856: f64, t7664: f64, t25890: f64, t113: f64, t2314: f64, t24980: f64, t24983: f64, t24988: f64, t24989: f64, t24993: f64, t24998: f64, t24999: f64, t25005: f64, t25007: f64, t25011: f64, t4073: f64, t4077: f64, t6517: f64, t652: f64, t672: f64, t7472: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25928, t25930, t25934, t25938, t25945, t25949) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1843(t25374, t25927, t1081, t1530, t28, t4303, t1649, t776, t868, t1877, t1915, t22959, t23290, t25013, t2522, t25354, t25358, t25372, t25397, t25892, t25898, t25901, t25905, t25921, t6666, t6670, t6841, t6848, t7541, t7649, t7656);
        let (t25950, t25958, t25962) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1844(t28, t265, t504, t25882, t1409, t1972, t25949, t3966, t52, t607, t6856, t7664, t25890, t113, t2314, t24980, t24983, t24988, t24989, t24993, t24998, t24999, t25005, t25007, t25011, t4073, t4077, t6517, t652, t672, t7472, dens_threshold, rho1, zeta_threshold);
    (t25928, t25930, t25934, t25938, t25945, t25950, t25958, t25962)
}
