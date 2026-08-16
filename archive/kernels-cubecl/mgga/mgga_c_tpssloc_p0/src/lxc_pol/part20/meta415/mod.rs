//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta415 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1822;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta415<F: Float>(t1539: F, t3132: F, t3071: F, t3041: F, t1616: F, t2776: F, t13969: F, t4584: F, t1041: F, t4589: F, t12652: F, t4583: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t14121, t14122, t14125, t14126, t14129, t14130, t14134, t14136, t14137, t14139, t14142) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1822::<F>(t1539, t3132, t3071, t3041, t1616, t2776, t13969, t4584, t1041, t4589, t12652, t4583);
    (t14121, t14122, t14125, t14126, t14129, t14130, t14134, t14136, t14137, t14139, t14142)
}
