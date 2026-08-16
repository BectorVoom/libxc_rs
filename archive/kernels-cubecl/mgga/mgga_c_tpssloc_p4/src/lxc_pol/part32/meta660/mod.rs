//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta660 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2089;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2090;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta660<F: Float>(t27654: F, t491: F, t1235: F, t8034: F, t27434: F, t85639: F, t27821: F, t24600: F, t7301: F, t27798: F, t4935: F, t24615: F, t24637: F, t8009: F, t24588: F, t8020: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t94354, t94358, t94363, t94365, t94369, t94374, t94378) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2089::<F>(t27654, t491, t1235, t8034, t27434, t85639, t27821, t24600, t7301, t27798, t4935, t24615);
        let (t94391, t94395) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2090::<F>(t24637, t8009, t24588, t8020);
    (t94354, t94358, t94363, t94365, t94369, t94374, t94378, t94391, t94395)
}
