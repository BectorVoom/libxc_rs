//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta307 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1554;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1555;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta307<F: Float>(t1100: F, t11258: F, t1107: F, t410: F, t417: F, t11244: F, t11137: F, t11139: F, t11141: F, t11143: F, t11150: F, t11156: F, t11165: F, t11174: F, t11230: F, t11233: F, t11245: F, t11228: F, t1118: F, t1099: F, t1097: F, t3311: F, t409: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11259, t11261, t11265, t11266, t11268) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1554::<F>(t1100, t11258, t1107, t410, t417, t11244, t11137, t11139, t11141, t11143, t11150, t11156, t11165, t11174, t11230, t11233, t11245);
        let (t11269, t11270, t11272, t11274, t11275) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1555::<F>(t11228, t11268, t1118, t1099, t1097, t3311, t409);
    (t11259, t11261, t11265, t11266, t11269, t11270, t11272, t11274, t11275)
}
