//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta347 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1649;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta347<F: Float>(t12132: F, t17: F, t3826: F, t592: F, t1285: F, t2225: F, t2371: F, t3691: F, t1294: F, t9494: F, t2535: F, t12121: F, t12123: F, t12125: F, t12128: F, t12131: F, t9853: F, t9859: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t12133, t12134, t12135, t12136, t12137, t12138, t12139, t12141, t12142, t12143, t12144) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1649::<F>(t12132, t17, t3826, t592, t1285, t2225, t2371, t3691, t1294, t9494, t2535, t12121, t12123, t12125, t12128, t12131, t9853, t9859);
    (t12133, t12134, t12135, t12136, t12137, t12138, t12139, t12141, t12142, t12143, t12144)
}
