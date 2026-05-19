//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 880/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk880<F: Float>(t12826: F, t6313: F, t12840: F, t6305: F, t2268: F, t3137: F, t7930: F, t2765: F, t9152: F, t39791: F, t39794: F, t39798: F) -> (F, F, F, F, F, F, F) {
    let t42806 = F::cast_from(0.45528010617081839357e0_f64) * t6313 * t12826;
    let t42808 = F::cast_from(0.85365019907028448797e-1_f64) * t6305 * t12840;
    let t42811 = F::cast_from(0.85365019907028448797e-1_f64) * t2268 * t7930 * t3137;
    let t42814 = F::cast_from(0.85365019907028448797e-1_f64) * t2268 * t2765 * t9152;
    let t42815 = F::cast_from(0.23712505529730124666e-2_f64) * t39791;
    let t42816 = F::cast_from(0.23712505529730124666e-2_f64) * t39794;
    let t42817 = F::cast_from(0.23712505529730124666e-2_f64) * t39798;
    (t42806, t42808, t42811, t42814, t42815, t42816, t42817)
}
