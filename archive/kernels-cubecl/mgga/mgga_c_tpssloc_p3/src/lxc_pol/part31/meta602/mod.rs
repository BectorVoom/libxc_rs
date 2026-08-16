//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta602 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1847;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta602<F: Float>(t87338: F, t87341: F, t87347: F, t87363: F, t87401: F, t87411: F, t87443: F, t87463: F, t87477: F, t87487: F, t87565: F, t87581: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t92649, t92650, t92653, t92657, t92675, t92679, t92697, t92705, t92710, t92713, t92729, t92738) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1847::<F>(t87338, t87341, t87347, t87363, t87401, t87411, t87443, t87463, t87477, t87487, t87565, t87581);
    (t92649, t92650, t92653, t92657, t92675, t92679, t92697, t92705, t92710, t92713, t92729, t92738)
}
