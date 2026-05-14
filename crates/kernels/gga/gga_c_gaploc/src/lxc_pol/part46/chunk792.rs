//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 792/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk792<F: Float>(t39778: F, t12826: F, t6313: F, t12840: F, t6305: F, t2268: F, t3137: F, t7930: F, t2765: F, t9152: F, t39791: F, t39794: F, t39798: F, t42778: F, t42782: F, t42786: F, t42790: F, t42793: F, t42795: F, t42797: F, t42799: F, t42802: F, t42803: F) -> (F,) {
    let t42804 = 0.47425011059460249332e-2 * t39778;
    let t42806 = 0.45528010617081839357e0 * t6313 * t12826;
    let t42808 = 0.85365019907028448797e-1 * t6305 * t12840;
    let t42811 = 0.85365019907028448797e-1 * t2268 * t7930 * t3137;
    let t42814 = 0.85365019907028448797e-1 * t2268 * t2765 * t9152;
    let t42815 = 0.23712505529730124666e-2 * t39791;
    let t42816 = 0.23712505529730124666e-2 * t39794;
    let t42817 = 0.23712505529730124666e-2 * t39798;
    let t42818 = -0.3983700928994660944e0 * t42778 + 0.6829201592562275904e0 * t42782 - 0.3414600796281137952e0 * t42786 + t42790 + t42793 - t42795 - t42797 - t42799 - t42802 - t42803 + t42804 + t42806 - t42808 - t42811 - t42814 + t42815 + t42816 - t42817;
    (t42818,)
}
