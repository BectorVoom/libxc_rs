//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta192 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk853;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk854;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk855;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk856;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta192(t2749: f64, t868: f64, t261: f64, t2751: f64, t193: f64, t202: f64, t9793: f64, t9797: f64, t9820: f64, t9824: f64, t9872: f64, t9876: f64, t9881: f64, t9884: f64, t9887: f64, t9890: f64, t9894: f64, t9896: f64, t9853: f64, t9859: f64, t9900: f64, t9903: f64, t9907: f64, t9911: f64, t9914: f64, t9917: f64, t9921: f64, t9923: f64, t9925: f64, t9928: f64, t9931: f64, t9934: f64, t10125: f64, t10138: f64, t225: f64, t3023: f64, t1053: f64, t68: f64, t1065: f64, t3175: f64, t3021: f64, t3206: f64, t3174: f64, t1887: f64, t337: f64, t615: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10140, t10143, t10147) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk853(t2749, t868, t261, t2751, t193, t202, t9793, t9797, t9820, t9824, t9872, t9876, t9881, t9884, t9887, t9890, t9894, t9896);
        let t10148 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk854(t9853, t9859, t9900, t9903, t9907, t9911, t9914, t9917, t9921, t9923, t9925, t9928, t9931, t9934);
        let t10150 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk855(t10125, t10138, t10147, t10148);
        let (t10160, t10163, t10165, t10167, t10170, t10182, t10186) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk856(t225, t3023, t1053, t68, t1065, t3175, t3021, t3206, t3174, t1887, t337, t615);
    (t10140, t10143, t10150, t10160, t10163, t10165, t10167, t10170, t10182, t10186)
}
