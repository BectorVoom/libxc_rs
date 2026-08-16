//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta465 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1819;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1820;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1821;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta465<F: Float>(t221: F, t2987: F, t1926: F, t344: F, t381: F, t225: F, t1054: F, t883: F, t1065: F, t607: F, t6733: F, t6691: F, t1955: F, t3175: F, t10165: F, t6686: F, t6712: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23326, t23327) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1819::<F>(t221, t2987, t1926);
        let (t23328, t23329) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1820::<F>(t344, t381, t225);
        let (t23330, t23331, t23332, t23333, t23336, t23337, t23341, t23346) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1821::<F>(t1054, t883, t1065, t607, t23329, t381, t6733, t6691, t1955, t3175, t10165, t6686, t6712);
    (t23326, t23327, t23328, t23329, t23330, t23331, t23332, t23333, t23336, t23337, t23341, t23346)
}
