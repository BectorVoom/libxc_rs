//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta336 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1200;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1201;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta336(t39494: f64, t761: f64, t152: f64, t185: f64, t39097: f64, t153: f64, t157: f64, t39842: f64, t10140: f64, t10143: f64, t2374: f64, t39354: f64, t193: f64, t202: f64, t2522: f64, t39529: f64, t40760: f64, t40762: f64, t40764: f64, t40766: f64, t40768: f64, t40769: f64, t40772: f64, t40777: f64, t776: f64) -> (f64, f64, f64, f64, f64) {
        let (t40779, t40782, t40784, t40785, t40790) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1200(t39494, t761, t152, t185, t39097, t153, t157, t39842, t10140, t10143, t2374, t39354);
        let t40791 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1201(t193, t202, t2522, t39529, t40760, t40762, t40764, t40766, t40768, t40769, t40772, t40777, t40779, t40782, t40784, t40785, t40790, t776);
    (t40779, t40782, t40784, t40790, t40791)
}
