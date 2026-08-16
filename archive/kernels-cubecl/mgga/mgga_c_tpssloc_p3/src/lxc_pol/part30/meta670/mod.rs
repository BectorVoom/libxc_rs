//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta670 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2099;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta670<F: Float>(t22643: F, t7691: F, t81195: F, t26502: F, t532: F, t22573: F, t7684: F, t2018: F, t40611: F, t2022: F, t5381: F, t26509: F, t580: F) -> (F, F, F, F, F, F) {
        let (t91548, t91620, t91655, t91686, t91813, t91816) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2099::<F>(t22643, t7691, t81195, t26502, t532, t22573, t7684, t2018, t40611, t2022, t5381, t26509, t580);
    (t91548, t91620, t91655, t91686, t91813, t91816)
}
