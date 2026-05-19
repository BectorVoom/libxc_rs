//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 933/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk933<F: Float>(t2617: F, t3626: F, t7810: F, t3614: F, t5241: F, t2679: F, t9805: F, t2676: F, t36612: F, t13525: F, t2089: F, t13617: F, t15751: F) -> (F, F, F, F, F) {
    let t45826 = t7810 * t3626 * t2617;
    let t45828 = t5241 * t3614;
    let t45830 = t9805 * t45828 * t2679;
    let t45831 = F::cast_from(0.57514388930881124514e0_f64) * t45830;
    let t45837 = F::cast_from(0.11916829983950142223e0_f64) * t36612 * t2676;
    let t45842 = t2089 * t13525;
    let t45848 = F::cast_from(0.27606906686822939767e2_f64) * t15751 * t13617;
    (t45826, t45831, t45837, t45842, t45848)
}
