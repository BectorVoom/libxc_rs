//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta370 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1717;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta370<F: Float>(t157: F, t9929: F, t4196: F, t9726: F, t10143: F, t1530: F, t2430: F, t4205: F, t1409: F, t750: F, t607: F, t4194: F) -> (F, F, F, F, F, F, F, F) {
        let (t12908, t12910, t12914, t12915, t12922, t12923, t12924, t12926) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1717::<F>(t157, t9929, t4196, t9726, t10143, t1530, t2430, t4205, t1409, t750, t607, t4194);
    (t12908, t12910, t12914, t12915, t12922, t12923, t12924, t12926)
}
