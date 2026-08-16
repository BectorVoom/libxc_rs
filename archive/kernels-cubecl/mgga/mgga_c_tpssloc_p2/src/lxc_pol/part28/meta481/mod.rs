//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta481 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1695;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta481<F: Float>(t4072: F, t89: F, t22751: F, t7692: F, t22666: F, t7691: F, t6888: F, t5187: F, t6890: F, t6889: F, t1834: F, t214: F) -> (F, F, F, F, F, F, F, F) {
        let (t26179, t26184, t26186, t26187, t26189, t26190, t26191, t26193) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1695::<F>(t4072, t89, t22751, t7692, t22666, t7691, t6888, t5187, t6890, t6889, t1834, t214);
    (t26179, t26184, t26186, t26187, t26189, t26190, t26191, t26193)
}
