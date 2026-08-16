//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta464 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1815;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1816;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1817;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta464(t2752: f64, t6665: f64, t10143: f64, t1914: f64, t25: f64, t2749: f64, t606: f64, t868: f64, t2745: f64, t1877: f64, t1915: f64, t2249: f64, t22951: f64, t22959: f64, t22961: f64, t22964: f64, t22968: f64, t23286: f64, t2522: f64, t4314: f64, t6542: f64, t6666: f64, t6670: f64, t6671: f64, t6699: f64, t986: f64, t3206: f64, t6705: f64, t6704: f64, t1922: f64, t3016: f64, t2261: f64, t337: f64, t1887: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t23290 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1815(t2752, t6665);
        let t23295 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1816(t10143, t1914);
        let (t23296, t23299, t23302, t23309) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1817(t25, t2749, t606, t868, t2745, t1877, t1915, t2249, t22951, t22959, t22961, t22964, t22968, t23286, t23290, t23295, t2522, t4314, t6542, t6666, t6670, t6671);
        let (t23310, t23313, t23314, t23317, t23322, t23323) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1818(t6699, t986, t3206, t6705, t6704, t1922, t3016, t2261, t337, t1887);
    (t23290, t23295, t23296, t23299, t23302, t23309, t23310, t23313, t23314, t23317, t23322, t23323)
}
