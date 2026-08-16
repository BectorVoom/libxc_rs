//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta657 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1940;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1941;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta657(t232: f64, t25119: f64, t58557: f64, t815: f64, t22690: f64, t5527: f64, t81792: f64, t841: f64, t16805: f64, t1898: f64, t249: f64, t236: f64, t5584: f64, t23109: f64, t2632: f64, t81914: f64, t23110: f64, t5611: f64, t5587: f64, t81886: f64, t23041: f64, t5619: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98770, t98774, t98777, t98779) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1940(t232, t25119, t58557, t815, t22690, t5527, t81792, t841, t16805, t1898, t249, t236, t5584);
        let (t98782, t98787, t98791, t98796, t98798) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1941(t23109, t2632, t81914, t98779, t23110, t232, t236, t5611, t5587, t81886, t23041, t5619);
    (t98770, t98774, t98777, t98782, t98787, t98791, t98796, t98798)
}
