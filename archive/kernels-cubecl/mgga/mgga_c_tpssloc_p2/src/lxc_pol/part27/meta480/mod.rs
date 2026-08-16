//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta480 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1854;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta480<F: Float>(t381: F, t883: F, t6743: F, t14227: F, t6800: F, t23384: F, t6790: F, t1949: F, t3010: F, t6805: F, t986: F, t3016: F) -> (F, F, F, F, F, F, F) {
        let (t23634, t23635, t23637, t23642, t23644, t23647, t23650) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1854::<F>(t381, t883, t6743, t14227, t6800, t23384, t6790, t1949, t3010, t6805, t986, t3016);
    (t23634, t23635, t23637, t23642, t23644, t23647, t23650)
}
