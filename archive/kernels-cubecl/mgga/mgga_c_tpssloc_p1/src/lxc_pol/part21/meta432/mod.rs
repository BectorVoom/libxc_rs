//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta432 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1966;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta432<F: Float>(t15338: F, t3451: F, t3447: F, t14818: F, t14781: F, t14710: F, t11211: F, t11213: F, t11215: F, t11217: F, t11487: F, t14713: F, t14766: F, t14779: F, t14784: F, t14787: F, t14790: F, t14793: F, t14796: F, t14799: F) -> (F, F, F, F, F, F) {
        let (t15339, t15341, t15347, t15348, t15349, t15357) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1966::<F>(t15338, t3451, t3447, t14818, t14781, t14710, t11211, t11213, t11215, t11217, t11487, t14713, t14766, t14779, t14784, t14787, t14790, t14793, t14796, t14799);
    (t15339, t15341, t15347, t15348, t15349, t15357)
}
