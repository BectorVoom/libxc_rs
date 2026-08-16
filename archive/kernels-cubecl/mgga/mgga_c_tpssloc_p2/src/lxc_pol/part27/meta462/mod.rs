//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta462 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1811;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta462<F: Float>(t23237: F, t6555: F, t6552: F, t2379: F, t6554: F, t6553: F, t23035: F, t6547: F, t6568: F, t23030: F, t6563: F, t6567: F, t794: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t23238, t23239, t23241, t23242, t23243, t23249, t23250, t23252, t23253) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1811::<F>(t23237, t6555, t6552, t2379, t6554, t6553, t23035, t6547, t6568, t23030, t6563, t6567, t794);
    (t23238, t23239, t23241, t23242, t23243, t23249, t23250, t23252, t23253)
}
