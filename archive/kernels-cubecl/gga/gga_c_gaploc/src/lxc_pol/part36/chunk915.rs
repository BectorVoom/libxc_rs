//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 915/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk915<F: Float>(t2268: F, t2765: F, t9152: F, t39791: F, t39794: F, t39798: F, t42778: F, t42782: F, t42786: F, t42790: F, t42793: F, t42795: F, t42797: F, t42799: F, t42802: F, t42803: F, t42804: F, t42806: F, t42808: F, t42811: F) -> F {
    let t42814 = F::cast_from(0.85365019907028448797e-1_f64) * t2268 * t2765 * t9152;
    let t42815 = F::cast_from(0.23712505529730124666e-2_f64) * t39791;
    let t42816 = F::cast_from(0.23712505529730124666e-2_f64) * t39794;
    let t42817 = F::cast_from(0.23712505529730124666e-2_f64) * t39798;
    let t42818 = -F::cast_from(0.3983700928994660944e0_f64) * t42778 + F::cast_from(0.6829201592562275904e0_f64) * t42782 - F::cast_from(0.3414600796281137952e0_f64) * t42786 + t42790 + t42793 - t42795 - t42797 - t42799 - t42802 - t42803 + t42804 + t42806 - t42808 - t42811 - t42814 + t42815 + t42816 - t42817;
    t42818
}
