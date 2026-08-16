//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta282 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1326;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta282<F: Float>(t816: F, t9612: F, t2632: F, t776: F, t2678: F, t815: F, t836: F, t812: F, t2649: F, t2617: F, t2642: F, t1891: F, t67: F) -> (F, F, F, F, F, F, F) {
        let (t9613, t9627, t9632, t9638, t9639, t9642, t9645) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1326::<F>(t816, t9612, t2632, t776, t2678, t815, t836, t812, t2649, t2617, t2642, t1891, t67);
    (t9613, t9627, t9632, t9638, t9639, t9642, t9645)
}
