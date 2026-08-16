//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta597 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2118;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2119;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta597<F: Float>(t13797: F, t1597: F, t13783: F, t340: F, t4548: F, t698: F, t973: F, t10224: F, t4522: F, t13895: F, t2960: F, t1599: F, t2402: F, t13908: F, t2986: F, t344: F, t43052: F, t4343: F, t2978: F, t4338: F, t697: F, t43053: F, t4514: F, t1592: F, t42891: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t48221, t48279, t48293, t48321, t48329, t48336) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2118::<F>(t13797, t1597, t13783, t340, t4548, t698, t973, t10224, t4522, t13895, t2960, t1599, t2402);
        let (t48339, t48374, t48379, t48382, t48397) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2119::<F>(t13908, t2960, t2986, t344, t43052, t4343, t2978, t4338, t697, t43053, t4514, t1592, t42891, t973);
    (t48221, t48279, t48293, t48321, t48329, t48336, t48339, t48374, t48379, t48382, t48397)
}
