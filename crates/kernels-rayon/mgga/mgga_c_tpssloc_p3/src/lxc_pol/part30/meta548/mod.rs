//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta548 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1900;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1901;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1902;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta548(t109: f64, t1873: f64, t28002: f64, t4028: f64, t7467: f64, t5493: f64, t88: f64, t7676: f64, t22473: f64, t5464: f64, t5488: f64, t6530: f64, t22469: f64, t27166: f64, t1268: f64, t1458: f64, t24999: f64, t27993: f64, t27996: f64, t28001: f64, t6517: f64, t510: f64, t652: f64, t7685: f64, t7756: f64, t89: f64, t1874: f64, t7458: f64, t7461: f64, t7468: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28004, t28006, t28007, t28009, t28011, t28017) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1900(t109, t1873, t28002, t4028, t7467, t5493, t88, t7676, t22473, t5464, t5488, t6530, t22469, t27166);
        let t28020 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1901(t1268, t28017, t1458, t24999, t27993, t27996, t28001, t28004, t28006, t28009, t28011, t5493, t6517);
        let (t28025, t28027, t28029, t28030, t28032, t28034, t28036) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1902(t28017, t510, t652, t7685, t7756, t5493, t89, t1874, t7458, t7461, t4028, t7468);
    (t28007, t28017, t28020, t28025, t28027, t28029, t28030, t28032, t28034, t28036)
}
