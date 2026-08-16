//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta509 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1909;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1910;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1911;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1912;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1913;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta509(t22960: f64, t25365: f64, t193: f64, t1962: f64, t10143: f64, t25: f64, t1530: f64, t868: f64, t606: f64, t4303: f64, t1408: f64, t776: f64, t1877: f64, t1915: f64, t2219: f64, t22959: f64, t23290: f64, t25013: f64, t25015: f64, t25021: f64, t25024: f64, t25028: f64, t2522: f64, t25354: f64, t25358: f64, t6542: f64, t6666: f64, t6670: f64, t6671: f64, t7475: f64, t7541: f64, t7545: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25366, t25372) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1909(t22960, t25365, t193, t1962);
        let t25373 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1910(t10143, t25);
        let t25374 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1911(t1530, t868);
        let (t25375, t25377, t25381, t25385, t25392, t25397) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1912(t25373, t25374, t1530, t606, t25, t4303, t1408, t776, t868, t1877, t1915, t2219);
        let t25398 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1913(t1408, t1877, t1915, t22959, t23290, t25, t25013, t25015, t25021, t25024, t25028, t2522, t25354, t25358, t25366, t25372, t25375, t25377, t25381, t25385, t25392, t25397, t606, t6542, t6666, t6670, t6671, t7475, t7541, t7545);
    (t25366, t25372, t25373, t25374, t25375, t25377, t25381, t25385, t25392, t25397, t25398)
}
