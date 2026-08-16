//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2564/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2564<F: Float>(t10482: F, t3120: F, t10470: F, t11064: F, t381: F, t1057: F, t49864: F, t3199: F, t49649: F, t11045: F, t14538: F, t225: F) -> (F, F, F, F, F, F) {
    let t50510 = t10482 * t3120;
    let t50516 = t10470 * t11064 * t381;
    let t50535 = t49864 * t1057;
    let t50592 = t49649 * t3199;
    let t50610 = t10470 * t11045 * t381;
    let t50622 = t14538 * t225;
    (t50510, t50516, t50535, t50592, t50610, t50622)
}
