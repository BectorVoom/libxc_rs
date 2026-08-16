//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta395 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1200;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta395<F: Float>(t2517: F, t5520: F, t12945: F, t4205: F, t32: F, t5519: F, t5398: F, t707: F, t16616: F, t2535: F, t2371: F, t41115: F, t5593: F) -> (F, F, F, F, F, F, F) {
        let (t57897, t57960, t57973, t57992, t58021, t58057, t58421) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1200::<F>(t2517, t5520, t12945, t4205, t32, t5519, t5398, t707, t16616, t2535, t2371, t41115, t5593);
    (t57897, t57960, t57973, t57992, t58021, t58057, t58421)
}
