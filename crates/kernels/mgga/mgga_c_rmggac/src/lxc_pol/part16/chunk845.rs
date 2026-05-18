//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 845/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk845<F: Float>(t2186: F, t8587: F, t9000: F, t9128: F, t7244: F, t9165: F, t2160: F, t638: F, t8850: F, t8854: F, t5055: F, t7769: F) -> (F, F, F, F, F, F) {
    let t41960 = t2186 * t8587;
    let t41977 = t9128 * t9000;
    let t41979 = t7244 * t9165;
    let t42023 = t638 * t2160 * t8850;
    let t42026 = t638 * t2160 * t8854;
    let t42034 = t5055 * t7769;
    (t41960, t41977, t41979, t42023, t42026, t42034)
}
