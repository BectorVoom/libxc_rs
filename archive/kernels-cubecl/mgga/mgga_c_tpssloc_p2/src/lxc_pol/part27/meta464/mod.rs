//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta464 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1815;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1816;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1817;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta464<F: Float>(t2752: F, t6665: F, t10143: F, t1914: F, t25: F, t2749: F, t606: F, t868: F, t2745: F, t1877: F, t1915: F, t2249: F, t22951: F, t22959: F, t22961: F, t22964: F, t22968: F, t23286: F, t2522: F, t4314: F, t6542: F, t6666: F, t6670: F, t6671: F, t6699: F, t986: F, t3206: F, t6705: F, t6704: F, t1922: F, t3016: F, t2261: F, t337: F, t1887: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t23290 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1815::<F>(t2752, t6665);
        let t23295 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1816::<F>(t10143, t1914);
        let (t23296, t23299, t23302, t23309) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1817::<F>(t25, t2749, t606, t868, t2745, t1877, t1915, t2249, t22951, t22959, t22961, t22964, t22968, t23286, t23290, t23295, t2522, t4314, t6542, t6666, t6670, t6671);
        let (t23310, t23313, t23314, t23317, t23322, t23323) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1818::<F>(t6699, t986, t3206, t6705, t6704, t1922, t3016, t2261, t337, t1887);
    (t23290, t23295, t23296, t23299, t23302, t23309, t23310, t23313, t23314, t23317, t23322, t23323)
}
