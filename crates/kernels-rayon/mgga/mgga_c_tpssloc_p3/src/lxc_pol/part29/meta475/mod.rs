//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta475 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1811;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1812;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta475(t24503: f64, t67: f64, t1864: f64, t6509: f64, t7255: f64, t2109: f64, t22489: f64, t7245: f64, t9239: f64, t22550: f64, t9231: f64, t33: f64, t7254: f64, t2240: f64, t1860: f64, t2110: f64, t22493: f64, t22519: f64, t22527: f64, t22531: f64, t22534: f64, t22537: f64, t22546: f64, t22549: f64, t6486: f64, t6492: f64, t6495: f64, t7246: f64, t7256: f64, t7259: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24504, t24505, t24508, t24511, t24514) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1811(t24503, t67, t1864, t6509, t7255, t2109, t22489, t7245, t9239);
        let (t24517, t24520, t24525, t24526, t24541) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1812(t2109, t22550, t7245, t9231, t33, t7254, t2240, t1860, t2110, t22493, t22519, t22527, t22531, t22534, t22537, t22546, t22549, t24505, t24508, t24511, t24514, t6486, t6492, t6495, t7246, t7256, t7259);
    (t24504, t24505, t24508, t24511, t24514, t24517, t24520, t24525, t24526, t24541)
}
