//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta475 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1811;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1812;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta475<F: Float>(t24503: F, t67: F, t1864: F, t6509: F, t7255: F, t2109: F, t22489: F, t7245: F, t9239: F, t22550: F, t9231: F, t33: F, t7254: F, t2240: F, t1860: F, t2110: F, t22493: F, t22519: F, t22527: F, t22531: F, t22534: F, t22537: F, t22546: F, t22549: F, t6486: F, t6492: F, t6495: F, t7246: F, t7256: F, t7259: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t24504, t24505, t24508, t24511, t24514) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1811::<F>(t24503, t67, t1864, t6509, t7255, t2109, t22489, t7245, t9239);
        let (t24517, t24520, t24525, t24526, t24541) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1812::<F>(t2109, t22550, t7245, t9231, t33, t7254, t2240, t1860, t2110, t22493, t22519, t22527, t22531, t22534, t22537, t22546, t22549, t24505, t24508, t24511, t24514, t6486, t6492, t6495, t7246, t7256, t7259);
    (t24504, t24505, t24508, t24511, t24514, t24517, t24520, t24525, t24526, t24541)
}
