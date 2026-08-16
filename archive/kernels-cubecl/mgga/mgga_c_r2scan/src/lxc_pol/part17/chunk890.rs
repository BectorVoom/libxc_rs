//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 890/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk890<F: Float>(t1567: F, t3052: F, t2124: F, t2591: F, t8778: F, t360: F, t3055: F, t6359: F, t494: F, t6363: F, t9317: F, t8820: F) -> (F, F, F, F, F, F) {
    let t9524 = t1567 * t3052;
    let t9526 = t2124 * t9524 * t2591;
    let t9529 = t8778 * t2591;
    let t9530 = t360 * t9529;
    let t9533 = t6359 * t3055;
    let t9534 = t6363 * t494;
    let t9536 = t2124 * t9533 * t9534;
    let t9540 = t2124 * t9317 * t2591;
    let t9543 = t8820 * t9534;
    (t9526, t9529, t9530, t9536, t9540, t9543)
}
