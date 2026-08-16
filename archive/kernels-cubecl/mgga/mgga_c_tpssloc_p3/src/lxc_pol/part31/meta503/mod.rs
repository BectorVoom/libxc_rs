//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta503 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1699;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta503<F: Float>(t225: F, t567: F, t6434: F, t214: F, t1985: F, t6460: F, t6906: F, t6889: F, t6347: F, t6890: F, t6888: F, t26193: F, t7691: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t28199, t28200, t28201, t28205, t28206, t28207, t28209, t28210, t28211, t28213) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1699::<F>(t225, t567, t6434, t214, t1985, t6460, t6906, t6889, t6347, t6890, t6888, t26193, t7691);
    (t28199, t28200, t28201, t28205, t28206, t28207, t28209, t28210, t28211, t28213)
}
