//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta692 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2204;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2205;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2206;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2207;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta692(t5544: f64, t606: f64, t16662: f64, t25: f64, t2752: f64, t28447: f64, t28248: f64, t776: f64, t22960: f64, t10143: f64, t1408: f64, t25374: f64, t1530: f64, t584: f64, t86730: f64, t25372: f64, t5397: f64, t868: f64, t81547: f64, t5660: f64, t17109: f64, t1877: f64, t1915: f64, t22959: f64, t23290: f64, t25013: f64, t2522: f64, t28249: f64, t28448: f64, t28459: f64, t6666: f64, t6670: f64, t6671: f64, t81483: f64, t5664: f64, t25373: f64, t23168: f64, t28288: f64, t10109: f64, t1888: f64, t23270: f64, t5636: f64, t865: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98046, t98050, t98054, t98058, t98059, t98065) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2204(t5544, t606, t16662, t25, t2752, t28447, t28248, t776, t22960, t10143, t1408, t25374);
        let (t98071, t98075, t98079, t98082, t98086) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2205(t1530, t584, t86730, t25372, t5397, t868, t28248, t81547, t5660, t606, t17109, t25);
        let t98090 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2206(t1877, t1915, t22959, t23290, t25013, t2522, t25372, t28249, t28448, t28459, t5397, t606, t6666, t6670, t6671, t81483, t98046, t98050, t98054, t98059, t98065, t98071, t98075, t98079, t98082, t98086);
        let (t98091, t98094, t98102, t98103, t98111, t98112, t98117, t98122) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2207(t5664, t606, t5397, t776, t5660, t868, t25373, t28248, t23168, t28288, t10109, t1888, t23270, t5636, t865);
    (t98054, t98058, t98071, t98090, t98091, t98094, t98102, t98103, t98111, t98112, t98117, t98122)
}
