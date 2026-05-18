//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 742/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk742<F: Float>(t7033: F, t7070: F, t7101: F, t7136: F, t106: F, t167: F, t2100: F, t2107: F, t2189: F, t3461: F, t670: F, t6960: F, t6964: F, t6968: F, t6977: F, t6978: F, t6983: F, t708: F) -> (F, F) {
    let t7138 = t7033 + t7070 + t7101 + t7136;
    let t7142 = F::new(0.27818116767324025134e1) * t106 * t6960 * t167 - F::new(0.83454350301972075402e1) * t106 * t6964 * t708 + F::new(0.16690870060394415081e2) * t106 * t6968 * t2107 - F::new(0.83454350301972075402e1) * t106 * t2100 * t2189 - F::new(0.1669087006039441508e2) * t106 * t6977 * t6978 + F::new(0.16690870060394415081e2) * t3461 * t6983 - F::new(0.27818116767324025134e1) * t106 * t670 * t7138;
    (t7138, t7142)
}
