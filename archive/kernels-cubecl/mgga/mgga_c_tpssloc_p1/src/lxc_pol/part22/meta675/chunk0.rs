//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2233/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2233<F: Float>(t13950: F, t4644: F, t10508: F, t248: F, t3130: F, t5873: F, t17611: F, t3114: F, t10904: F, t17667: F, t1040: F, t17877: F) -> (F, F, F, F, F) {
    let t61659 = t4644 * t13950;
    let t61663 = t3130 * t248 * t10508 * t5873;
    let t61665 = t3114 * t17611;
    let t61675 = t10904 * t17667;
    let t61677 = t17877 * t1040;
    (t61659, t61663, t61665, t61675, t61677)
}
