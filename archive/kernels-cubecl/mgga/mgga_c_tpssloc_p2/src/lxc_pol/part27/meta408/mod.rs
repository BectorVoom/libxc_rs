//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta408 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1697;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta408<F: Float>(t16205: F, t550: F, t1343: F, t820: F, t12365: F, t1827: F, t12300: F, t1799: F, t3734: F, t12351: F, t12418: F, t1351: F) -> (F, F, F, F, F, F, F, F) {
        let (t16206, t16208, t16211, t16214, t16215, t16217, t16224, t16225) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1697::<F>(t16205, t550, t1343, t820, t12365, t1827, t12300, t1799, t3734, t12351, t12418, t1351);
    (t16206, t16208, t16211, t16214, t16215, t16217, t16224, t16225)
}
