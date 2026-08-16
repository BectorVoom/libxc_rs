//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta630 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2042;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta630<F: Float>(t1512: F, t81824: F, t23041: F, t4236: F, t23040: F, t4166: F, t831: F, t4191: F, t81749: F, t4240: F, t23069: F, t4159: F) -> (F, F, F, F, F, F, F) {
        let (t87248, t87256, t87261, t87263, t87271, t87273, t87291) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2042::<F>(t1512, t81824, t23041, t4236, t23040, t4166, t831, t4191, t81749, t4240, t23069, t4159);
    (t87248, t87256, t87261, t87263, t87271, t87273, t87291)
}
