//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1383;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1384;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta345<F: Float>(t1036: F, t4617: F, t10422: F, t4574: F, t3070: F, t1597: F, t4509: F, t10189: F, t344: F, t4343: F, t2986: F, t134: F, t2978: F, t4338: F, t10190: F, t4514: F, t10213: F, t60: F, t135: F, t340: F, t4548: F, t973: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13758, t13765, t13767, t13769, t13782, t13783) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1383::<F>(t1036, t4617, t10422, t4574, t3070, t1597, t4509, t10189, t344, t4343, t2986, t134, t2978);
        let (t13787, t13790, t13797, t13798, t13825) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1384::<F>(t13783, t344, t4338, t2986, t10190, t4514, t10213, t60, t135, t340, t4548, t973);
    (t13758, t13765, t13767, t13769, t13782, t13783, t13787, t13790, t13797, t13798, t13825)
}
