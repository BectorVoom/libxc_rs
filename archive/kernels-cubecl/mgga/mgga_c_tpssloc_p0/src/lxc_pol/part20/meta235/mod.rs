//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta235 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1331;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1332;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta235<F: Float>(t849: F, t9601: F, t2697: F, t2707: F, t241: F, t6589: F, t67: F, t820: F, t9458: F, t2613: F, t68: F, t816: F, t2553: F, t776: F) -> (F, F, F, F, F, F, F) {
        let (t9602, t9604, t9607, t9609, t9612) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1331::<F>(t849, t9601, t2697, t2707, t241, t6589, t67, t820, t9458, t2613, t68);
        let (t9613, t9616) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1332::<F>(t816, t9612, t2553, t776);
    (t9602, t9604, t9607, t9609, t9612, t9613, t9616)
}
