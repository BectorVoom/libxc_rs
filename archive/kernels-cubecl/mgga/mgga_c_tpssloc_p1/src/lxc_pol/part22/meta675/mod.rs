//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta675 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2233;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2234;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta675<F: Float>(t13950: F, t4644: F, t10508: F, t248: F, t3130: F, t5873: F, t17611: F, t3114: F, t10904: F, t17667: F, t1040: F, t17877: F, t3109: F, t135: F, t17737: F, t973: F, t10949: F, t17607: F, t3053: F, t3047: F, t5904: F, t18030: F, t3103: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t61659, t61663, t61665, t61675, t61677) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2233::<F>(t13950, t4644, t10508, t248, t3130, t5873, t17611, t3114, t10904, t17667, t1040, t17877);
        let (t61695, t61699, t61705, t61708, t61710, t61713) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2234::<F>(t17611, t3109, t135, t17737, t973, t10949, t17667, t17607, t3053, t3047, t5904, t18030, t3103);
    (t61659, t61663, t61665, t61675, t61677, t61695, t61699, t61705, t61708, t61710, t61713)
}
