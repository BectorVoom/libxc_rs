//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta539 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1800;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1801;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta539<F: Float>(t81763: F, t849: F, t6620: F, t9612: F, t23132: F, t2617: F, t23133: F, t2707: F, t131: F, t23121: F, t9537: F, t236: F, t81613: F, t23098: F, t22822: F, t281: F, t6589: F, t23124: F, t23076: F, t6597: F, t22690: F, t2379: F, t841: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t81764, t81766, t81769, t81770, t81772, t81782, t81783) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1800::<F>(t81763, t849, t6620, t9612, t23132, t2617, t23133, t2707, t131, t23121, t9537, t236, t81613);
        let (t81785, t81788, t81789, t81792, t81795) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1801::<F>(t23098, t81782, t81783, t22822, t281, t6589, t23124, t23076, t6597, t22690, t2379, t841);
    (t81764, t81766, t81769, t81770, t81772, t81782, t81783, t81785, t81788, t81789, t81792, t81795)
}
